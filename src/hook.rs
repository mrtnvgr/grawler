use git2::Repository;
use std::{
    fs::{self, File},
    io::Write,
    os::unix::fs::PermissionsExt,
};

static HOOK: &str = "#!/bin/sh\nexec grawler check";

pub fn setup() {
    eprintln!("Creating a git pre-commit hook...");

    let hooks_dir = Repository::discover(".")
        .map(|x| x.path().join("hooks"))
        .unwrap();

    fs::create_dir_all(&hooks_dir).expect("Failed to create a dir for hooks");

    let hook_path = hooks_dir.join("pre-commit");

    if hook_path.exists() {
        log::warn!("The pre-commit hook already exists");
    }

    let hook_bytes = HOOK.as_bytes();
    let mut hook = File::create(&hook_path).expect("Failed to create a hook");
    hook.write_all(hook_bytes)
        .expect("Failed to write to a hook");

    let mut perms = hook
        .metadata()
        .map(|x| x.permissions())
        .expect("Failed to get permissions from hook");
    perms.set_mode(0o755);

    fs::set_permissions(hook_path, perms).expect("Failed to set new perms");

    log::info!("Success! :)");
}
