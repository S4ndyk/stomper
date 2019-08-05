use std::error::Error;
use std::io::prelude::*;

pub struct Huffman;

impl super::Compressor for Huffman {
    /// Not yet implemented
    fn encode(input: &mut impl Read, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Not yet implemented
    fn decode(input: &mut impl Read, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
