use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use execute::{command, Execute};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{
    fs::{self, OpenOptions},
    path::Path,
};
use walkdir::WalkDir;

pub fn create_stalker_dir(path: &Path) {
    match fs::create_dir_all(path) {
        Ok(_) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("Successfully created stalker instance at {}\n", path.display())),
                ResetColor
                ) {
                Ok(_) => {},
                Err(e) => eprintln!("Error printing stalker directory creation output on create_dir_all function: {}", e)
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("Error creating stalker instance: {}\n", e)),
                ResetColor
                ) {
                Ok(_) => {},
                Err(e) => eprintln!("Error printing stalker directory creation error output on create_dir_all function: {}", e)
            }
        }
    }
}

pub fn create_stalk_list(stalker_instance: &Path) {
    match fs::File::create(stalker_instance.join("stalklist.txt")) {
        Ok(_) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print("Successfully created stalklist\n"),
                ResetColor
            ) {
                Ok(_) => {}
                Err(e) => eprintln!(
                    "Error printing stalklist creation output on create_stalk_list function: {}",
                    e
                ),
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error creating stalklist: {}\n", e)),
                ResetColor
                ) {
                Ok(_) => {},
                Err(e) => eprintln!("Error printing stalklist creation error output on create_stalk_list function: {}", e)
            }
        }
    }
}

pub fn update_stalk_list(stalker_instance: &Path, input_path: &String) {
    match OpenOptions::new()
        .append(true)
        .open(stalker_instance.join("stalklist.txt"))
    {
        // file variable is made mutable because write! macro takes a mutable handle
        Ok(mut file) => {
            match writeln!(file, "{}", input_path) {
                Ok(_) => {
                    match execute!(
                        stdout(),
                        SetForegroundColor(Color::Green),
                        Print(format!("Successfully added {} to stalklist\n", input_path)),
                        ResetColor
                        ) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Error printing stalklist update output on update_stalk_list function: {}", e)
                    }
                }
                Err(e) => {
                    match execute!(
                        stdout(),
                        SetForegroundColor(Color::Red),
                        Print(format!("Error adding {} to stalklist: {}\n", input_path, e)),
                        ResetColor
                        ) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Error printing stalklist update output on update_stalk_list function: {}", e)
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                    stdout(),
                    SetForegroundColor(Color::Red),
                    Print(format!("Error opening stalklist at {}: {}\n", stalker_instance.display(), e)),
                    ResetColor
                    ) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Error printing stalklist update file opening error output on update_stalk_list function: {}", e)
                }
        }
    }
}

pub fn list_action_list(stalker_instance: &Path) {
    match fs::File::open(stalker_instance.join("actionlist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Blue),
                            Print(format!("{}\n", item)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing actionlist item output on list_action_list function: {}", e)
                        }
                    }
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error reading line(s): {}\n", e)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing actionlist item error output on list_action_list function: {}", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error opening actionlist at {}: {}\n", stalker_instance.display(), e)),
                ResetColor
                )
            {
                Ok(_) => {},
                Err(e) => eprintln!("Error printing actionlist opening error output on list_action_list function: {}", e)
            }
        }
    }
}

pub fn list_stalk_list(stalker_instance: &Path) {
    match fs::File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Blue),
                            Print(format!("{}\n", item)),
                            ResetColor
                            ){
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing stalklist item output on list_stalk_list function: {}", e)
                        }
                    }
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error reading line(s): {}\n", e)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing stalklist item error output on list_stalk_list function: {}", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!(
                    "Error opening stalklist at {}: {}\n",
                    stalker_instance.display(),
                    e
                )),
                ResetColor
            ) {
                Ok(_) => {}
                Err(e) => eprintln!(
                    "Error printing stalklist opening error output on list_stalk_list function: {}",
                    e
                ),
            }
        }
    }
}

pub fn remove_from_stalklist(stalker_instance: &Path, path_to_remove: Vec<&String>) {
    let mut stalklist_item: Vec<String> = Vec::new();

    match fs::File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => stalklist_item.push(item),
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error reading line(s): {}\n", e)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing stalklist item removal error output on remove_from_stalklist function: {}", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error opening stalklist at {}: {}\n", stalker_instance.join("stalklist.txt").display(), e)),
                ResetColor
                ) {
                Ok(_) => {},
                Err(e) => eprintln!("Error printing stalklist opening error output on remove_from_stalklist function: {}", e)
            }
        }
    }

    if !stalker_instance.join("stalklist.txt").exists() {
        match execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error opening stalklist at {}: Cannot find stalklist.txt\n", stalker_instance.join("stalklist.txt").display())),
            ResetColor
            ) {
            Ok(_) => {},
            Err(e) => eprintln!("Error printing stalklist existence checking error output on remove_from_stalklist function: {}", e)
        }
        return;
    } else if stalklist_item.is_empty() {
        match execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error deleting item from stalklist at {}: stalklist.txt is empty\n", stalker_instance.join("stalklist.txt").display())),
            ResetColor
            ) {
            Ok(_) => {},
            Err(e) => eprintln!("Error printing stalklist empty checking error output on remove_from_stalklist function: {}", e)
        }
        return;
    }

    for item in path_to_remove {
        stalklist_item.retain(|i| i != item);
        match execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("Successfully removed {} from stalklist\n", item)),
            ResetColor
        ) {
            Ok(_) => {}
            Err(e) => eprintln!(
                "Error printing remove stalklist item output on remove_from_stalklist function: {}",
                e
            ),
        }
    }

    match fs::File::create(stalker_instance.join("stalklist.txt")) {
        Ok(mut file) => {
            for path in stalklist_item {
                match writeln!(file, "{}", path) {
                    Ok(_) => {}
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error writing path(s) to stalklist at {}: {}\n", stalker_instance.display(), e)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing stalklist line rewrite error output on remove_from_stalklist function: {}", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error re-writing stalklist at {}: {}\n", stalker_instance.display(), e)),
                ResetColor
                ) {
                Ok(_) => {}
                Err(e) => eprintln!("Error printing stalklist file rewrite error output on remove_from_stalklist function: {}", e)
            }
        }
    }
}

pub fn create_commands(stalker_instance: &Path) {
    match fs::File::create(stalker_instance.join("actionlist.txt")) {
        Ok(_) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print("Successfully created actionlist\n"),
                ResetColor
                ) {
               Ok(_) => {},
               Err(e) => eprintln!("Error printing actionlist creation error output on create_commands function: {}", e)
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error creating actionlist at {}: {}\n", stalker_instance.display(), e)),
                ResetColor
                ) {
               Ok(_) => {},
               Err(e) => eprintln!("Error printing failed actionlist creation error output on create_commands function: {}", e)
            }
        }
    }
}

pub fn update_commands(stalker_instance: &Path, command: &String) {
    match OpenOptions::new()
        .append(true)
        .open(stalker_instance.join("actionlist.txt"))
    {
        Ok(mut file) => match writeln!(file, "{}", command) {
            Ok(_) => match execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("Successfully added {} to actionlist.\n", command)),
                ResetColor,
            ) {
                Ok(_) => {}
                Err(e) => eprintln!(
                    "Error printing actionlist update output on update_commands function: {}",
                    e
                ),
            },
            Err(e) => match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error adding {} to actionlist: {}\n", command, e)),
                ResetColor
            ) {
                Ok(_) => {}
                Err(e) => eprintln!(
                    "Error printing actionlist update error output on update_commands function: {}",
                    e
                ),
            },
        },
        Err(e) => {
            match execute!(
                    stdout(),
                    SetForegroundColor(Color::Red),
                    Print(format!("Error opening actionlist at {}: {}\n", stalker_instance.display(), e)),
                    ResetColor
                    ) {
                   Ok(_) => {},
                   Err(e) => eprintln!("Error printing actionlist opening error output on update_commands function: {}",e)
                }
        }
    }
}

pub fn remove_from_actionlist(stalker_instance: &Path, action_to_remove: Vec<&String>) {
    let mut action_item: Vec<String> = Vec::new();

    match fs::File::open(stalker_instance.join("actionlist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => action_item.push(item),
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error reading line(s): {}\n", e)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing actionlist item removal error output on remove_from_actionlist function: {}", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error opening actionlist at {}: {}\n", stalker_instance.join("actionlist.txt").display(), e)),
                ResetColor
                ) {
                Ok(_) => {},
                Err(e) => eprintln!("Error printing actionlist opening error output on remove_from_ actionlist function: {}", e)
            }
        }
    }

    if !stalker_instance.join("actionlist.txt").exists() {
        match execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error opening actionlist at {}: Cannot find actionlist.txt\n", stalker_instance.join("actionlist.txt").display())),
            ResetColor
            ) {
            Ok(_) => {},
            Err(e) => eprintln!("Error printing actionlist existence checking error output on remove_from_actionlist function: {}", e)
        }
        return;
    } else if action_item.is_empty() {
        match execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error deleting item from actionlist at {}: actionlist.txt is empty\n", stalker_instance.join("actionlist.txt").display())),
            ResetColor
            ) {
            Ok(_) => {},
            Err(e) => eprintln!("Error printing actionlist empty checking error output on remove_from_actionlist function: {}", e)
        }
        return;
    }

    for item in action_to_remove {
        action_item.retain(|i| i != item);
        match execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("Successfully removed {} from actionlist\n", item)),
            ResetColor
        ) {
            Ok(_) => {}
            Err(e) => eprintln!(
                "Error printing remove actionlist item output on remove_from_actionlist function: {}",
                e
            ),
        }
    }

    match fs::File::create(stalker_instance.join("actionlist.txt")) {
        Ok(mut file) => {
            for path in action_item{
                match writeln!(file, "{}", path) {
                    Ok(_) => {}
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error writing path(s) to actionlist at {}: {}\n", stalker_instance.display(), e)),
                            ResetColor
                            ) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error printing actionlist line rewrite error output on remove_from_actionlist function: {}", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Error re-writing actionlist at {}: {}\n", stalker_instance.display(), e)),
                ResetColor
                ) {
                Ok(_) => {}
                Err(e) => eprintln!("Error printing actionlist file rewrite error output on remove_from_actionlist function: {}", e)
            }
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
                                    match execute!(
                                        stdout(),
                                        SetForegroundColor(Color::Red),
                                        Print(format!("Error getting true path {}: {}\n", path, e)),
                                        ResetColor
                                        ) {
                                       Ok(_) => {},
                                       Err(e) => {
                                           eprintln!("Error printing true path error output on run_stalker function: {}", e);
                                           return ;
                                       }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error reading stalklist at {}: {}\n", stalker_instance.display(), e)),
                            ResetColor
                            ) {
                           Ok(_) => {},
                           Err(e) => {
                               eprintln!("Error printing stalklist reading error output on run_stalker function: {}", e);
                               return ;
                           }
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!(
                    "Error opening stalklist at {}: {}\n",
                    stalker_instance.display(),
                    e
                )),
                ResetColor
            ) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                    "Error printing stalklist opening error output at run_stalker function: {}",
                    e
                );
                    return ;
                }
            }
        }
    }

    match File::open(stalker_instance.join("actionlist.txt")) {
        Ok(file) => {
            for actions in BufReader::new(file).lines() {
                match actions {
                    Ok(action) => command_vec.push(action),
                    Err(e) => {
                        match execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print(format!("Error reading actionlist at {}: {}\n", stalker_instance.display(), e)),
                            ResetColor
                            ) {
                           Ok(_) => {},
                           Err(e) => {
                               eprintln!("Error printing actionlist reading error output at run_stalker function: {}", e);
                               return ;
                           }
                        }
                    }
                }
            }
        }
        Err(e) => {
            match execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!(
                    "Error opening actionlist at {}: {}\n",
                    stalker_instance.display(),
                    e
                )),
                ResetColor
            ) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                    "Error printing actionlist opening error output at run_stalker function: {}",
                    e
                );
                    return ;
                }
            }
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
                            println!("{}", String::from_utf8(output.stdout).unwrap());
                        }
                    }
                }
                Err(e) => {
                    match execute!(
                        stdout(),
                        SetForegroundColor(Color::Red),
                        Print(format!("Error receiving event: {}\n", e)),
                        ResetColor
                        ) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error printing event receiving error output on run_stalker function: {}", e);
                            return ;
                        }
                    }
                }
            }
        }
    } 
}
