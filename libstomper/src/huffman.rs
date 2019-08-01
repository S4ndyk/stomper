use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub struct Huffman;

impl super::Compressor for Huffman {
    fn compress(&self, input: &File) -> Result<PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from("temp"))
    }
    fn decompress(&self, input: &File) -> Result<PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from("temp"))
    }
}
