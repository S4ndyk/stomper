pub mod args;
use std::error;
use std::fs::File;
pub mod huffman;
pub mod lzw;
use args::Args;

pub fn run(args: &Args) -> Result<(), Box<dyn error::Error>> {
    let input = File::open(&args.input)?;
    let output = &args.output;
    let compressor = &args.compressor;
    match args.decompress {
        true => compressor.decompress(&input, output),
        false => compressor.compress(&input, output),
    }
}

pub trait Compressor {
    fn compress(&self, i: &File, o: &Option<String>) -> Result<(), Box<dyn error::Error>>;
    fn decompress(&self, i: &File, o: &Option<String>) -> Result<(), Box<dyn error::Error>>;
}
