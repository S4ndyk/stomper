use std::error;
use std::fs::File;

pub struct Huffman;

impl super::Compressor for Huffman {
    fn compress(&self, mut i: &File, o: &Option<String>) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
    fn decompress(&self, mut i: &File, o: &Option<String>) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
