use structopt::StructOpt;
extern crate stomper;

fn main() {
    let args = stomper::args::Args::from_args();
    if let Err(e) = stomper::run(&args) {
        eprintln!("An error has occurred: {}", e);
    }
}
