use std::io::{BufWriter, Write};
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
            "{}{}{}Error creating stalker instance: {}",
            style::Bold,
            style::Italic,
            color::Fg(color::Red),
            e
        ),
    }
}

// TODO: Find a way to open file only once so I/O isn't going to be expensive at big input (e.g.
// 100, 1000, 10000+ paths)
pub fn create_stalk_list(stalker_instance: PathBuf, input_path: &String) {
    match fs::write(stalker_instance.join("stalklist.txt"), input_path) {
        Ok(_) => println!(
            "{}{}Successfully added {} to stalklist.",
            style::Bold,
            color::Fg(color::Green),
            input_path
        ),
        Err(e) => eprintln!(
            "{}{}{}Error adding {} to stalklist: {}",
            style::Bold,
            style::Italic,
            color::Fg(color::Red),
            input_path,
            e
        ),
    }
}

pub fn update_stalk_list(stalker_instance: PathBuf, input_path: &String) {
    match OpenOptions::new()
        .append(true)
        .open(stalker_instance.join("stalklist.txt"))
    {
        Ok(mut file) => match write!(file, "{}", input_path) {
            Ok(_) => println!(
                "{}{}Successfully added {} to stalklist.",
                style::Bold,
                color::Fg(color::Green),
                input_path
            ),
            Err(e) => eprintln!(
                "{}{}{}Error adding {} to stalklist: {}",
                style::Bold,
                style::Italic,
                color::Fg(color::Red),
                input_path,
                e
            ),
        },
        Err(e) => {
            eprintln!(
                "{}{}{}Error opening stalklist at {}: {}",
                style::Bold,
                style::Italic,
                color::Fg(color::Red),
                input_path,
                e
            )
        }
    }
}
