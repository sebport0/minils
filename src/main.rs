use chrono::{DateTime, Utc};
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path_to_read = args[1].clone();
    let metadata = fs::metadata(&path_to_read).unwrap();

    if metadata.is_dir() {
        let contents =
            fs::read_dir(path_to_read).expect("Something went wrong when reading from path!");

        for entry in contents {
            if let Ok(entry) = entry {
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
        }
    } else {
        panic!("The path provided is not a directory.");
    }
}
