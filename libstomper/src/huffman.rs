use std::error::Error;
use std::io::prelude::*;

pub struct Huffman;

impl super::Compressor for Huffman {
    fn encode(input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn decode(input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
