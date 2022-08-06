use std::io::{Write, BufReader, BufRead};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};
use termion::{color, style};

pub fn create_stalker_dir(path: &PathBuf) {
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

pub fn create_stalk_list(stalker_instance: &PathBuf) {
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

pub fn update_stalk_list(stalker_instance: &PathBuf, input_path: &String) {
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

pub fn list_stalk_list(stalker_instance: &PathBuf) {
    match fs::File::open(stalker_instance.join("stalklist.txt")) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(item) => {
                        println!("{}", item);
                    },
                    Err(e) => {
                        eprintln!("Error reading line(s): {}", e);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error opening stalklist at {}: {}", stalker_instance.display(), e);
        }
    }
}
