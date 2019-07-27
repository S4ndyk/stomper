use std::fs::File;

pub struct Huffman;

impl super::Compressor for Huffman {
    fn compress(mut input: &File, output: &Option<String>) {}

    fn decompress(mut input: &File, output: &Option<String>) {}
}
