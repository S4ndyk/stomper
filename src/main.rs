use structopt::StructOpt;

mod lib;
use lib::{run, Args};

fn main() {
    let args = Args::from_args();
    run(&args);
}
