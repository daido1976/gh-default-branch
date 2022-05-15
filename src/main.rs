mod cmd;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "gh default-branch")]
#[clap(about = "GitHub CLI extension to show & rename the default branch", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show default branch
    Show {
        /// Show only the branch name (e.g. main)
        #[clap(short, long)]
        name: bool,
    },
    /// Rename default branch
    #[clap(arg_required_else_help = true)]
    Rename { name: String },
}

fn main() {
    run()
}

fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show { name } => {
            cmd::show(name);
        }
        Commands::Rename { name } => cmd::rename(&name),
    }
}
