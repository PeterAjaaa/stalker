use std::{fs, path::PathBuf};
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
