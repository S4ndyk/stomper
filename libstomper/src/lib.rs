//! `libstomper` is a collection of compression algorithms
//! For claritys sake all algorithm are represented as structs that implement the Compressor trait.
//! A compressor is only responsible for reading and writing data and

use std::error::Error;
use std::io::prelude::*;
pub mod huffman;
pub mod lzw;

/// Defines functions compression algorithms must implement
pub trait Compressor {
    fn encode<R: Read + Seek, W: Write + Seek>(input: &mut R, output: &mut W) -> Result<(), Box<dyn Error>>;
    fn decode<R: Read + Seek, W: Write + Seek>(input: &mut R, output: &mut W) -> Result<(), Box<dyn Error>>;
}
