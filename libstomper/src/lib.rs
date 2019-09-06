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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{prelude::*};
    use tempfile::*;
    use super::huffman::Huffman;
    use super::Compressor;
    use super::lzw::LZW;

    #[test]
    fn  log_huffman_performance() {
        let mut log = File::create("../documentation/huffman_stats.txt").unwrap();
        let mut big1 = File::open("../testfiles/big1.txt").unwrap();
        let mut big2 = File::open("../testfiles/big2.txt").unwrap();
        let mut big3 = File::open("../testfiles/big3.txt").unwrap();
        let mut temp1 = tempfile().unwrap();
        let mut temp2 = tempfile().unwrap();
        let mut temp3 = tempfile().unwrap();
        Huffman::encode(&mut big1, &mut temp1).unwrap();
        Huffman::encode(&mut big2, &mut temp2).unwrap();
        Huffman::encode(&mut big3, &mut temp3).unwrap();
        let orig1 = big1.metadata().unwrap().len(); 
        let orig2 = big2.metadata().unwrap().len(); 
        let orig3 = big3.metadata().unwrap().len(); 
        let comp1 = temp1.metadata().unwrap().len();
        let comp2 = temp2.metadata().unwrap().len();
        let comp3 = temp3.metadata().unwrap().len();
        let perc1: f64 = (comp1 as f64/orig1 as f64) * 100 as f64;
        let perc2: f64 = (comp2 as f64/orig2 as f64) * 100 as f64;
        let perc3: f64 = (comp3 as f64/orig3 as f64) * 100 as f64;
        let res1 = format!("\nbig1.txt: {} bytes\ncomp1: {} bytes\n{:.1}% of original\n", orig1, comp1, perc1);
        let res2 = format!("\nbig2.txt: {} bytes\ncomp2: {} bytes\n{:.1}% of original\n", orig2, comp2, perc2);
        let res3 = format!("\nbig3.txt: {} bytes\ncomp3: {} bytes\n{:.1}% of original\n", orig3, comp3, perc3);
        log.write(res1.as_bytes()).unwrap();
        log.write(res2.as_bytes()).unwrap();
        log.write(res3.as_bytes()).unwrap();
    }

    #[test]
    fn  log_lzw_performance() {
        let mut log = File::create("../documentation/lzw_stats.txt").unwrap();
        let mut big1 = File::open("../testfiles/big1.txt").unwrap();
        let mut big2 = File::open("../testfiles/big2.txt").unwrap();
        let mut big3 = File::open("../testfiles/big3.txt").unwrap();
        let mut temp1 = tempfile().unwrap();
        let mut temp2 = tempfile().unwrap();
        let mut temp3 = tempfile().unwrap();
        LZW::encode(&mut big1, &mut temp1).unwrap();
        LZW::encode(&mut big2, &mut temp2).unwrap();
        LZW::encode(&mut big3, &mut temp3).unwrap();
        let orig1 = big1.metadata().unwrap().len(); 
        let orig2 = big2.metadata().unwrap().len(); 
        let orig3 = big3.metadata().unwrap().len(); 
        let comp1 = temp1.metadata().unwrap().len();
        let comp2 = temp2.metadata().unwrap().len();
        let comp3 = temp3.metadata().unwrap().len();
        let perc1: f64 = (comp1 as f64/orig1 as f64) * 100 as f64;
        let perc2: f64 = (comp2 as f64/orig2 as f64) * 100 as f64;
        let perc3: f64 = (comp3 as f64/orig3 as f64) * 100 as f64;
        let res1 = format!("\norig1: {} bytes\ncomp1: {} bytes\n{:.1}% of original\n", orig1, comp1, perc1);
        let res2 = format!("\norig2: {} bytes\ncomp2: {} bytes\n{:.1}% of original\n", orig2, comp2, perc2);
        let res3 = format!("\norig3: {} bytes\ncomp3: {} bytes\n{:.1}% of original\n", orig3, comp3, perc3);
        log.write(res1.as_bytes()).unwrap();
        log.write(res2.as_bytes()).unwrap();
        log.write(res3.as_bytes()).unwrap();
    }
}