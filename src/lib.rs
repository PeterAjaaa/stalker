use execute::{command, Execute};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{
    fs::{self, OpenOptions},
    path::Path,
};
use termion::{color, style};
use walkdir::WalkDir;

pub fn create_stalker_dir(path: &Path) {
    match fs::create_dir_all(path) {
        Ok(_) => println!(
            "{}{}Successfully created stalker instance at {}",
            style::Bold,
            color::Fg(color::Green),
            path.display()
        ),
        Err(e) => eprintln!(
            "{}{}Error creating stalker instance: {}",
            style::Bold,
            color::Fg(color::Red),
            e
        ),
    }
}

pub fn create_stalk_list(stalker_instance: &Path) {
    match fs::File::create(stalker_instance.join("stalklist.txt")) {
        Ok(_) => println!(
            "{}{}Successfully created stalklist.",
            style::Bold,
            color::Fg(color::Green),
        ),
        Err(e) => eprintln!(
            "{}{}Error created stalklist: {}",
            style::Bold,
            color::Fg(color::Red),
            e
        ),
    }
}

pub fn update_stalk_list(stalker_instance: &Path, input_path: &String) {
    match OpenOptions::new()
        .append(true)
        .open(stalker_instance.join("stalklist.txt"))
    {
        // file variable is made mutable because write! macro takes a mutable handle
        Ok(mut file) => match writeln!(file, "{}", input_path) {
            Ok(_) => println!(
                "{}{}Successfully added {} to stalklist.",
                style::Bold,
                color::Fg(color::Green),
                input_path
            ),
            Err(e) => eprintln!(
                "{}{}Error adding {} to stalklist: {}",
                style::Bold,
                color::Fg(color::Red),
                input_path,
                e
            ),
        },
        Err(e) => {
            eprintln!(
                "{}{}Error opening stalklist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
            )
        }
    }
}

pub fn list_action_list(stalker_instance: &Path) {
    match fs::File::open(stalker_instance.join("actionlist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => {
                        println!("{}{}{}", style::Bold, color::Fg(color::Blue), item);
                    }
                    Err(e) => {
                        eprintln!(
                            "{}{}Error reading line(s): {}",
                            style::Bold,
                            color::Fg(color::Red),
                            e
                        )
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}Error opening actionlist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
            )
        }
    }
}

pub fn list_stalk_list(stalker_instance: &Path) {
    match fs::File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => {
                        println!("{}{}{}", style::Bold, color::Fg(color::Blue), item);
                    }
                    Err(e) => {
                        eprintln!(
                            "{}{}Error reading line(s): {}",
                            style::Bold,
                            color::Fg(color::Red),
                            e
                        )
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}Error opening stalklist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
            )
        }
    }
}

pub fn remove_from_list(stalker_instance: &Path, path_to_remove: Vec<&String>) {
    let mut stalklist_item: Vec<String> = Vec::new();

    match fs::File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => stalklist_item.push(item),
                    Err(e) => {
                        eprintln!(
                            "{}{}Error reading line(s): {}",
                            style::Bold,
                            color::Fg(color::Red),
                            e
                        )
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}Error opening stalklist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.join("stalklist.txt").display(),
                e
            )
        }
    }

    if !stalker_instance.join("stalklist.txt").exists() {
        eprintln!(
            "{}{}Error opening stalklist at {}: Cannot find stalklist.txt",
            style::Bold,
            color::Fg(color::Red),
            stalker_instance.join("stalklist.txt").display(),
            );
        return ;
    } else if stalklist_item.len() == 0 {
        eprintln!(
            "{}{}Error deleting item from stalklist at {}: stalklist.txt is empty",
            style::Bold,
            color::Fg(color::Red),
            stalker_instance.join("stalklist.txt").display(),
            );
        return ;
    }


    for item in path_to_remove {
        stalklist_item.retain(|i| i != item);
        println!(
            "{}{}Successfully removed {} from stalklist",
            style::Bold,
            color::Fg(color::Green),
            item
            );
    }

    match fs::File::create(stalker_instance.join("stalklist.txt")) {
        Ok(mut file) => {
            for path in stalklist_item {
                match writeln!(file, "{}", path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "{}{}Error writing path(s) to stalklist at {}: {}",
                            style::Bold,
                            color::Fg(color::Red),
                            stalker_instance.display(),
                            e
                            )
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}Error re-writing stalklist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
                )
        }
    }
}

pub fn create_commands(stalker_instance: &Path) {
    match fs::File::create(stalker_instance.join("actionlist.txt")) {
        Ok(_) => {
            println!(
                "{}{}Successfully created actionlist",
                style::Bold,
                color::Fg(color::Green)
                )
        }
        Err(e) => {
            eprint!(
                "{}{}Error creating actionlist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
                )
        }
    }
}

pub fn update_commands(stalker_instance: &Path, command: &String) {
    match OpenOptions::new()
        .append(true)
        .open(stalker_instance.join("actionlist.txt"))
        {
            Ok(mut file) => match writeln!(file, "{}", command) {
                Ok(_) => println!(
                    "{}{}Successfully added {} to actionlist.",
                    style::Bold,
                    color::Fg(color::Green),
                    command
                    ),
                Err(e) => eprintln!(
                    "{}{}Error adding {} to actionlist: {}",
                    style::Bold,
                    color::Fg(color::Red),
                    command,
                    e
                    ),
            },
            Err(e) => {
                eprintln!(
                    "{}{}Error opening actionlist at {}: {}",
                    style::Bold,
                    color::Fg(color::Red),
                    stalker_instance.display(),
                    e
                    )
            }
        }
}

pub fn run_stalker(stalker_instance: &Path) {
    let mut path_vec: Vec<PathBuf> = Vec::new();
    let mut command_vec: Vec<String> = Vec::new();

    match File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for paths in BufReader::new(file).lines() {
                match paths {
                    Ok(path) => {
                        for entry in WalkDir::new(&path) {
                            match entry {
                                Ok(true_path) => path_vec.push(true_path.path().to_owned()),
                                Err(e) => {
                                    eprintln!(
                                        "{}{}Error getting true path {}: {}",
                                        style::Bold,
                                        color::Fg(color::Red),
                                        path,
                                        e
                                        )
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{}{}Error reading stalklist at {}: {}",
                            style::Bold,
                            color::Fg(color::Red),
                            stalker_instance.display(),
                            e
                            )
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}Error opening stalklist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
                )
        }
    }

    match File::open(stalker_instance.join("actionlist.txt")) {
        Ok(file) => {
            for actions in BufReader::new(file).lines() {
                match actions {
                    Ok(action) => command_vec.push(action),
                    Err(e) => {
                        eprintln!(
                            "{}{}Error reading actionlist at {}: {}",
                            style::Bold,
                            color::Fg(color::Red),
                            stalker_instance.display(),
                            e
                            )
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}Error opening actionlist at {}: {}",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
                )
        }
    }

    for path in path_vec {
        let (tx, rx) = channel();
        let mut watcher =
            watcher(tx, Duration::from_secs(5)).expect("Error creating watcher object");
        watcher
            .watch(&path, RecursiveMode::Recursive)
            .expect("Error watching file");
        loop {
            match rx.recv() {
                Ok(event) => {
                    if event == DebouncedEvent::NoticeWrite(path.to_path_buf()) {
                        for raw_command in &command_vec {
                            let command_replaced = str::replace(
                                raw_command,
                                "{path}",
                                path.to_str().expect("Error substituting command"),
                                );
                            let mut actual_command = command(&command_replaced);
                            actual_command.stdout(Stdio::piped());
                            let output = actual_command.execute_output().unwrap();
                            println!("{}{}", color::Fg(color::Reset), style::Reset);
                            println!("{}", String::from_utf8(output.stdout).unwrap());
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{}{}Error receiving event: {}",
                        style::Bold,
                        color::Fg(color::Red),
                        e
                        )
                }
            }
        }
    }
}
