use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_read = args[1].clone();
    let metadata = fs::metadata(&path_to_read).unwrap();

    if metadata.is_dir() {
        let contents =
            fs::read_dir(path_to_read).expect("Something went wrong when reading from path!");

        for path in contents {
            println!("{}", path.unwrap().path().display());
        }
    } else {
        panic!("The path provided is not a directory.");
    }
}
