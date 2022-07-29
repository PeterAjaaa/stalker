use clap::{arg, Command};

fn main() {
    let app = Command::new("stalker")
        .version("0.1.0")
        .author("Peter <peterajaaa@gmail.com>")
        .about("stalker is a fast and simple file watcher and executor with git-like syntax.")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
            .about("Initialize a stalker instance")
            )
        .subcommand(
            Command::new("add")
            .about("Add path(s) to the stalk-list")
            .arg_required_else_help(true)
            .arg(
                arg!([PATH])
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                )
            )
        .get_matches();

    match app.subcommand() {
        Some(("init", _init_subcommand)) => {
            println!("stalker initialized...");
        }
        Some(("add", add_subcommand)) => {
            let paths: Vec<_> = add_subcommand.get_many::<String>("PATH").unwrap().collect();
            println!("{:?}", paths);
        }
        _ => () //Done because every subcommand should raise help on error.
    }
    }
