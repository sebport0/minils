use std::error::Error;

use chrono::{DateTime, Utc};
use std::fs;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Use a dir as an argument to list its contents.
pub struct Cli {
    /// List contents from path.
    #[structopt(parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    /// Show additional permissions, size, creation, last modified and last accessed dates
    /// for each entry.
    #[structopt(short, long)]
    pub list: bool,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_dir(cli.path).expect("Something went wrong when reading from path!");

    for entry in contents {
        print_entry(entry.unwrap(), cli.list);
    }

    Ok(())
}

fn print_entry(entry: DirEntry, list_flag: bool) {
    let path = entry.path();
    let path_last = path.iter().last().unwrap().to_str().unwrap();

    match list_flag {
        true => print_with_metadata(entry, path_last),
        false => println!("{}", path_last),
    };
}

fn print_with_metadata(entry: DirEntry, path_last: &str) {
    let metadata = entry.metadata().unwrap();
    let permissions = metadata.permissions().mode();
    let size = metadata.len();

    let created: DateTime<Utc> = metadata.created().unwrap().into();
    let created = created.date();

    let last_modified: DateTime<Utc> = metadata.modified().unwrap().into();
    let last_modified = last_modified.date();

    let last_accessed: DateTime<Utc> = metadata.accessed().unwrap().into();
    let last_accessed = last_accessed.date();

    println!(
        "{} {} {} {} {} {}",
        permissions, size, created, last_modified, last_accessed, path_last
    );
}
