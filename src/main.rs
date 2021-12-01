use std::process;
use structopt::StructOpt;

fn main() {
    let args = minils::Cli::from_args();

    if let Err(e) = minils::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
