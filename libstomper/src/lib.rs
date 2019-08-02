use std::error::Error;
use std::io::prelude::*;
pub mod huffman;
pub mod lzw;

pub trait Compressor {
    fn compress(&self, input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>>;
    fn decompress(&self, input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>>;
}
