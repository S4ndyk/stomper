//! Implementation of the Ziv-Lempel-Welch algorithm
//!
//! Right now data is encoded with 24-bit dictionary in little endian.
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::{collections::HashMap, error::Error, io::prelude::*};

 const MAX: u32 = 16777216;

pub struct LZW;

impl super::Compressor for LZW {
    /// Encodes data with Lempel-Ziv-Welch compression.
    fn encode(input: &mut impl Read, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut dict = LZW::init_dict();
        let mut current = String::new();
        let mut next = 257;
        for byte in input.bytes() {
            let c = byte.unwrap() as char;
            current.push(c);
            if let None = dict.get(&current) {
                dict.insert(current.clone(), next);
                next += 1;
                current.pop();
                let code = dict.get(&current).unwrap();
                output.write_u24::<LE>(*code)?;
                current = c.to_string();
            }
            if next >= MAX {
                dict = LZW::init_dict();
                next = 257;
            }
        }
        output.write_u24::<LE>(*dict.get(&current).unwrap())?;
        Ok(())
    }

    /// Decodes data with Lempel-Ziv-Welch compression.
    fn decode(input: &mut impl Read, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut dict = LZW::init_rev_dict();
        let mut prev = String::new();
        let mut next = 257;
        while let Ok(integer) = input.read_u24::<LE>() {
            if let None = dict.get(&integer) {
                let mut clone = prev.clone();
                clone.push(char_at(&prev, 0));
                dict.insert(integer, clone);
            }
            let current = dict.get(&integer).unwrap().clone();
            output.write(current.as_bytes())?;
            if !prev.is_empty() {
                let mut clone = prev.clone();
                clone.push(char_at(&current, 0));
                dict.insert(next, clone);
                next += 1;
            }
            prev = current;

            if next >= MAX {
                dict = LZW::init_rev_dict();
                next = 257;
            }
        }
        Ok(())
    }
}

impl LZW {
    /// initializes dictionary for encoding
    fn init_dict() -> HashMap<String, u32> {
        let mut dict = HashMap::new();
        for i in 0..256 {
            let c = i as u8 as char;
            dict.insert(c.to_string(), i);
        }
        dict
    }

    /// initializes dictionary for decoding
    fn init_rev_dict() -> HashMap<u32, String> {
        let mut dict = HashMap::new();
        for i in 0..256 {
            let c = i as u8 as char;
            dict.insert(i, c.to_string());
        }
        dict
    }
}

/// returns charatacter from string at index
fn char_at(s: &String, index: usize) -> char {
    s.as_bytes()[index] as char
}

#[cfg(test)]
mod tests {
    use super::super::Compressor;
    use super::LZW;
    use std::fs::File;
    use std::io::{prelude::*, SeekFrom};
    use tempfile::*;

    #[test]
    fn decomp_and_orig_are_same_no1() {
        let mut testfile = File::open("../testfiles/small.txt").unwrap();
        let mut comp = tempfile().unwrap();
        let mut decomp = tempfile().unwrap();

        LZW::encode(&mut testfile, &mut comp).unwrap();
        comp.seek(SeekFrom::Start(0)).unwrap();
        LZW::decode(&mut comp, &mut decomp).unwrap();

        let mut decomp_content = String::new();
        let mut testfile_content = String::new();

        decomp.seek(SeekFrom::Start(0)).unwrap();
        testfile.seek(SeekFrom::Start(0)).unwrap();

        decomp.read_to_string(&mut decomp_content).unwrap();
        testfile.read_to_string(&mut testfile_content).unwrap();

        assert_eq!(testfile_content, decomp_content);
    }

    #[test]
    #[ignore]
    fn compressed_file_is_smaller() {
        let mut testfile = File::open("../testfiles/big.txt").unwrap();
        let mut compressed = tempfile().unwrap();
        LZW::encode(&mut testfile, &mut compressed).unwrap();
        let testfile_meta = testfile.metadata().unwrap();
        let compressed_meta = compressed.metadata().unwrap();
        assert!(compressed_meta.len() < testfile_meta.len());
    }

    #[test]
    fn dict_has_corresponding_character() {
        let dict = LZW::init_dict();
        let num1: u32 = 65;
        let num2: u32 = 66;
        let num3: u32 = 67;
        let s1 = String::from("A");
        let s2 = String::from("B");
        let s3 = String::from("C");
        assert_eq!(dict.get(&s1), Some(&num1));
        assert_eq!(dict.get(&s2), Some(&num2));
        assert_eq!(dict.get(&s3), Some(&num3));
    }
}
