use std::fs::File;

pub struct LZW;

impl super::Compressor for LZW {
    fn compress(input: &File, output: &Option<String>) {}

    fn decompress(input: &File, output: &Option<String>) {}
}
