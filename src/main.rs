use clap::Parser;
use git2::Repository;

mod check;
mod hook;

#[derive(Parser)]
enum Commands {
    #[command(name = "check", about = "perform checks on urls in files")]
    Check(Check),
    #[command(name = "setup-hook", about = "set up a git pre-commit hook")]
    SetupHook,
}

#[derive(Parser)]
struct Check {
    #[arg(long, help = "show open urls")]
    show_open: bool,
    #[arg(long, help = "show urls for everyone")]
    show_all: bool,
    #[arg(long, help = "(debug) exit right after parsing")]
    offline: bool,
}

#[tokio::main]
async fn main() {
    cli_logger::init();
    let command = Commands::parse();

    assert!(
        Repository::discover(".").is_ok(),
        "Current directory is not in a git repository"
    );

    match command {
        Commands::Check(args) => check::perform(args).await,
        Commands::SetupHook => hook::setup(),
    }
}
