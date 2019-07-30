use super::{huffman::Huffman, lzw::LZW, Compressor};
use std::path::PathBuf;
use structopt::StructOpt;

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
