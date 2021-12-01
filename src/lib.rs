use std::error::Error;

use chrono::{DateTime, Utc};
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;

pub struct Config {
    pub path_to_read: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let path_to_read = match args.next() {
            Some(arg) => arg,
            None => return Err("Path to read argument is needed."),
        };

        Ok(Config { path_to_read })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_dir(config.path_to_read).expect("Something went wrong when reading from path!");

    for entry in contents {
        print_entry(entry.unwrap())
    }

    Ok(())
}

pub fn print_entry(entry: DirEntry) {
    if let Ok(metadata) = entry.metadata() {
        let permissions = metadata.permissions().mode();
        let size = metadata.len();

        let created: DateTime<Utc> = metadata.created().unwrap().into();
        let created = created.date();

        let last_modified: DateTime<Utc> = metadata.modified().unwrap().into();
        let last_modified = last_modified.date();

        let last_accessed: DateTime<Utc> = metadata.accessed().unwrap().into();
        let last_accessed = last_accessed.date();

        let path = entry.path();
        let path_last = path.iter().last().unwrap().to_str().unwrap();

        println!(
            "{} {} {} {} {} {}",
            permissions, size, created, last_modified, last_accessed, path_last
        );
    } else {
        panic!(
            "Something went wrong when reading metadata for {:?}",
            entry.path()
        );
    }
}
