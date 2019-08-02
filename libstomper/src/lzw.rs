use byteorder::{WriteBytesExt, LE};
use std::{collections::HashMap, error::Error, io::prelude::*};

pub struct LZW;

impl super::Compressor for LZW {
    fn compress(&self, input: impl Read, mut output: impl Write) -> Result<(), Box<dyn Error>> {
        let mut dict = LZW::init_dict();
        let mut current = String::new();
        let mut next = 257;
        for byte in input.bytes() {
            let c = byte? as char;
            current.push(c);
            if let None = dict.get(&current) {
                dict.insert(current.clone(), next);
                next += 1;
                current.pop();
                let code = dict.get(&current).unwrap();
                output.write_u32::<LE>(*code)?;
                current = c.to_string();
            }
        }
        output.write_u32::<LE>(*dict.get(&current).unwrap())?;
        Ok(())
    }

    fn decompress(&self, input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl LZW {
    fn init_dict() -> HashMap<String, u32> {
        let mut dict = HashMap::new();
        for i in 0..256 {
            let c = i as u8 as char;
            dict.insert(c.to_string(), i);
        }
        dict
    }
}

#[cfg(test)]
mod tests {
    use super::LZW;

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

    #[test]
    fn compress() {}
}
