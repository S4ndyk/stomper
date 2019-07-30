use std::error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use libstomper::{lzw::LZW, huffman::Huffman, Compressor};

pub fn run(args: &Args) -> Result<(), Box<dyn error::Error>> {
    let input = File::open(&args.input)?;
    let output = &args.output;
    let compressor = &args.compressor;
    match args.decompress {
        true => compressor.decompress(&input, output),
        false => compressor.compress(&input, output),
    }
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long)]
    pub decompress: bool,

    #[structopt(parse(try_from_str = "parse_compressor"))]
    pub compressor: Box<dyn Compressor>,

    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short, long)]
    pub output: Option<String>,
}

fn parse_compressor(src: &str) -> Result<Box<dyn Compressor>, String> {
    match src {
        "lzw" => Ok(Box::new(LZW)),
        "huff" => Ok(Box::new(Huffman)),
        s => Err(format!("invalid compression type '{}'", s)),
    }
}