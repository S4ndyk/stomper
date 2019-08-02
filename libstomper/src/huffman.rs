use std::error::Error;
use std::io::prelude::*;

pub struct Huffman;

impl super::Compressor for Huffman {
    fn compress(&self, input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn decompress(&self, input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
