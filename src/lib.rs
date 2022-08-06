use std::io::{BufRead, BufReader, Write};
use std::{
    fs::{self, OpenOptions},
    path::Path,
};
use termion::{color, style};

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
                input_path,
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

pub fn remove_from_list(stalker_instance: &Path, path_to_remove: &String) {
    let mut temp_vec: Vec<String> = Vec::new();

    match fs::File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => {
                        if item != *path_to_remove {
                            temp_vec.push(item)
                        }
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

    match fs::File::create(stalker_instance.join("stalklist.txt")) {
        Ok(mut file) => {
            for path in temp_vec {
                match writeln!(file, "{}", path) {
                    Ok(_) => {
                        println!(
                            "{}{}Successfully removed {} from stalklist",
                            style::Bold,
                            color::Fg(color::Green),
                            path_to_remove
                        )
                    }
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
                "{}{}Error re-writing stalklist at {}: {}:",
                style::Bold,
                color::Fg(color::Red),
                stalker_instance.display(),
                e
            )
        }
    }
}
