use clap::{arg, Command};
use terminal_size::{Width, terminal_size};

fn main() {
    let app = Command::new("stalker")
        .term_width(if let Some((Width(w), _)) = terminal_size() { w as usize } else { 100 })
        .version("0.1.0")
        .author("Peter <peterajaaa@gmail.com>")
        .about("stalker is a fast and simple file watcher and executor with git-like syntax.")
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Initialize a stalker instance.
Default stalker instance will be made on HOME directory under '.stalker' folder."))
        .subcommand(
            Command::new("add")
                .about("Add path(s) to the stalk-list")
                .arg_required_else_help(true)
                .arg(
                    arg!([PATH])
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove path(s) from the stalk-list")
                .arg_required_else_help(true)
                .arg(
                    arg!([PATH])
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .subcommand(
            Command::new("do")
                .about("Specify operation(s) on item(s) in the stalk-list. Put the commands inside of quotes (\"\").
Each separate command should be placed inside of separate quotes (e.g. \"git add *\" \"git commit\").")
                .arg_required_else_help(true)
                .arg(
                    arg!([COMMANDS])
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .subcommand(
            Command::new("execute")
            .about("Execute commands specified by 'stalk do' on path(s) specified by 'stalk-add'. Commands will be executed whenever there's changes on the specified file(s) on the specified path(s).")
            )
        .get_matches();

    match app.subcommand() {
        Some(("init", _init_subcommand)) => {
            println!("stalker initialized...");
            // TODO: Insert function to initialize a new stalker instance.
        }
        Some(("add", add_subcommand)) => {
            /* Used Vec<&String> instead of Vec<_> to better show the data types within the vector.
             * Don't use Vec<&str> either, since &String does the deref coercion to &str by the compiler,
             * but both are essentially a different kind of data types.*/
            let paths: Vec<&String> = add_subcommand.get_many::<String>("PATH").unwrap().collect();
            // TODO: Insert function to add paths to stalk-list.
        }
        Some(("remove", remove_subcommand)) => {
            let paths: Vec<&String> = remove_subcommand
                .get_many::<String>("PATH")
                .unwrap()
                .collect();
            // TODO: Insert function to remove paths from stalk-list.
        }
        Some(("do", do_subcommand)) => {
            let commands: Vec<&String> = do_subcommand 
                .get_many::<String>("COMMANDS")
                .unwrap()
                .collect();
            // TODO: Insert function to execute commands on the shell.
        }
        Some(("execute", _execute_subcommand)) => {
            println!("Running stalker...");
            // TODO: Insert function to run the stalker instance.
        }
        _ => (), //Done because every subcommand should raise help on error.
    }
}
