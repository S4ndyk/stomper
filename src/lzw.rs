use std::error;
use std::fs::File;

pub struct LZW;

impl super::Compressor for LZW {
    fn compress(&self, mut i: &File, o: &Option<String>) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
    fn decompress(&self, mut i: &File, o: &Option<String>) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
