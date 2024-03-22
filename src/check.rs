use crate::Check;
use git2::{Config, Repository};
use gitfrog::Info as GitInfo;
use ignore::Walk;
use lazy_static::lazy_static;
use log_err::{LogErrOption, LogErrResult};
use regex::Regex;
use std::{fs::read_to_string, process::exit};
use url::Url;

static RE_PATTERN: &str = r"(?xs)
    (?P<tag>TODO|NOTE|WARN|FIXME)
    (?:\(@(?P<assignee>\w+)\))?
    :\s
    (?P<link>https?:\/\/\S+)\s*?
";

#[derive(Debug)]
struct LinkContext {
    path: String,
    assignee: Option<String>,
    link: Url,
}

impl LinkContext {
    const fn new(path: String, assignee: Option<String>, link: Url) -> Self {
        Self {
            path,
            assignee,
            link,
        }
    }
}

#[allow(clippy::future_not_send)]
pub async fn perform(args: Check) {
    lazy_static! {
        static ref RE: Regex = Regex::new(RE_PATTERN).unwrap();
    }

    eprintln!("Checking URLs...");

    let repo = Repository::discover(".").unwrap();
    let repo_root = repo.path().parent().unwrap();

    let global_user = Config::open_default().and_then(|x| x.get_string("user.name"));
    let repo_user = repo.config().and_then(|x| x.get_string("user.name"));
    let username = repo_user.or(global_user).ok();

    let files = Walk::new(repo_root)
        .filter_map(Result::ok)
        .filter(|x| x.file_type().is_some_and(|t| !t.is_dir()));

    let mut contexts = Vec::new();
    let cwd = std::env::current_dir().log_expect("Failed to get cwd");

    for file in files {
        let Ok(text) = read_to_string(file.path()) else {
            continue;
        };

        let rel_path = file
            .path()
            .strip_prefix(&cwd)
            .log_expect("Failed to get relative path");
        let path = rel_path.to_string_lossy().to_string();

        for capture in RE.captures_iter(&text) {
            let link = capture
                .name("link")
                .log_expect("Failed to find a link in a match");

            let link = Url::parse(link.as_str()).log_expect("Failed to parse a link");

            let assignee = capture.name("assignee").map(|x| x.as_str().to_owned());

            let context = LinkContext::new(path.clone(), assignee, link);
            contexts.push(context);
        }
    }

    if args.offline {
        exit(1);
    }

    // Fuses
    let mut exit_code = 0;

    let urls: Vec<&Url> = contexts.iter().map(|x| &x.link).collect();
    let results = GitInfo::from_urls_ref(urls).await;
    let res_count = results.len();
    for (result, context) in results
        .into_iter()
        .map(Result::unwrap)
        .filter(|x| !x.state.is_open() || args.show_open)
        .zip(contexts)
    {
        let is_assigned_to_us = context.assignee == username;
        let is_assigned_to_all = username.is_none() || context.assignee.is_none();
        let is_assigned = is_assigned_to_us || is_assigned_to_all;

        if !is_assigned && !args.show_all {
            continue;
        }

        println!();
        log::info!("Title: {}", result.title);
        log::info!("URL: {}", context.link);
        log::info!("File: {}", context.path);
        log::info!("Status: {}", result.state.as_str());
        if let Some(assignee) = context.assignee {
            log::warn!("Assignee: @{assignee}");
        }

        if !result.state.is_open() {
            exit_code = 1;
        }
    }

    if res_count > 0 {
        println!();
    }

    exit(exit_code);
}
