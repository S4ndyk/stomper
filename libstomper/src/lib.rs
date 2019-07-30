use std::{error::Error, fs::File};
pub mod huffman;
pub mod lzw;

pub trait Compressor {
    fn compress(&self, i: &File, o: &Option<String>) -> Result<(), Box<dyn Error>>;
    fn decompress(&self, i: &File, o: &Option<String>) -> Result<(), Box<dyn Error>>;
}
