use structopt::StructOpt;
mod lib;
use lib::{run, Args};

fn main() {
    let args = Args::from_args();
    if let Err(e) = run(args) {
        eprintln!("An error has occurred: {}", e);
    }
}
