use std::{error::Error, path::PathBuf, fs::File};
pub mod huffman;
pub mod lzw;

pub trait Compressor {
    fn compress(&self, input: &File) -> Result<PathBuf, Box<dyn Error>>;
    fn decompress(&self, input: &File) -> Result<PathBuf, Box<dyn Error>>;
}
