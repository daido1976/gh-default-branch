mod cmd;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "gh default-branch", version, about, long_about = None)]
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
        name_only: bool,
    },
    /// Rename default branch
    #[clap(arg_required_else_help = true)]
    Rename { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show { name_only } => {
            cmd::show(name_only);
        }
        Commands::Rename { name } => cmd::rename(&name),
    }
}
