use std::error::Error;
use std::io::prelude::*;
pub mod huffman;
pub mod lzw;

pub trait Compressor {
    fn encode(input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>>;
    fn decode(input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>>;
}
