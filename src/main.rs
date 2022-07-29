use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
    Add{
        #[clap(value_parser)]
        filepath: Option<String>,
    },
    Remove {
        #[clap(value_parser)]
        filepath: Option<String>,
    },
}

fn main () {
    let cli = Cli::parse();
}
