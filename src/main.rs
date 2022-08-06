use clap::{arg, Command};
use dirs::home_dir;
use stalker::{create_stalk_list, create_stalker_dir, update_stalk_list};
use terminal_size::{terminal_size, Width};
use termion::{color, style};

fn main() {
    let default_stalker_path = home_dir().unwrap().join(".stalker");
    let app = Command::new("stalker")
        .term_width(if let Some((Width(w), _)) = terminal_size() { w as usize } else { 100 })
        .version("0.1.0")
        .author("Peter <peterajaaa@gmail.com>")
        .about("stalker is a fast and simple file watcher and executor with git-like syntax.")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a stalker instance.
The stalker instance will be made on $HOME directory under '.stalker' folder.")
                .arg(
                    arg!([PATH])
                    .takes_value(true)
                )
            )
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
        Some(("init", _init_path)) => {
            println!(
                "{}{}stalker initialized...",
                style::Bold,
                color::Fg(color::Green)
            );
            create_stalker_dir(&default_stalker_path);
        }
        Some(("add", add_subcommand)) => {
            /* Used Vec<&String> instead of Vec<_> to better show the data types within the vector.
             * Don't use Vec<&str> either, since &String does the deref coercion to &str by the compiler,
             * but both are essentially a different kind of data types.*/
            /* Also a Vec<&String> is used because get_many() returns a reference to the actual
             * value.*/
            let paths: Vec<&String> = add_subcommand.get_many::<String>("PATH").unwrap().collect();
            if !default_stalker_path.exists() {
                eprintln!(
                    "{}{}Error creating stalklist. No stalker instance is found.",
                    style::Bold,
                    color::Fg(color::Red)
                );
                eprintln!(
                    "{}{}HINT: Run \"stalker init\" first before adding item(s) to the stalklist.",
                    style::Bold,
                    color::Fg(color::Yellow)
                );
            } else {
                create_stalk_list(&default_stalker_path);
                for path in paths {
                    update_stalk_list(&default_stalker_path, path);
                }
            }
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
            println!(
                "{}{}{}Running stalker...",
                style::Bold,
                style::Italic,
                color::Fg(color::Green)
            );
            // TODO: Insert function to run the stalker instance.
        }
        _ => (), //Done because every subcommand should raise help on error.
    }
}
