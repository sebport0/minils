use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_read = args[1].clone();
    let contents =
        fs::read_dir(path_to_read).expect("Something went wrong when reading from path!");

    for path in contents {
        println!("{}", path.unwrap().path().display());
    }
}
