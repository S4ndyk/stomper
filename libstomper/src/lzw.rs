use byteorder::{WriteBytesExt, LE};
use std::io::{prelude::*, BufReader, BufWriter};
use std::{collections::HashMap, error, fs::File, path::PathBuf};

pub struct LZW;

impl super::Compressor for LZW {
    ///Takes file and return path to compressed file
    fn compress(&self, input: &File) -> Result<PathBuf, Box<dyn error::Error>> {
        let mut dict = LZW::init_dict();
        let path = PathBuf::from("out.stmpd");
        let mut writer = BufWriter::new(File::create(&path)?);
        let reader = BufReader::new(input);

        let mut current = String::new();
        let mut next = 257;
        //Iterate over bytes in file
        for byte in reader.bytes() {
            //Turn byte to char
            let c = byte? as char;
            //Build string
            current.push(c);

            //If string not in dictionary
            if let None = dict.get(&current) {
                //Insert string to dictionary
                dict.insert(current.clone(), next);
                next += 1;
                current.pop();
                let code = dict.get(&current).unwrap();
                //write out unsigned 32 bit int as 4 bytes in little endian
                writer.write_u32::<LE>(*code)?;
                current = c.to_string();
            }
        }
        writer.write_u32::<LE>(*dict.get(&current).unwrap())?;
        Ok(path)
    }

    fn decompress(&self, i: &File) -> Result<PathBuf, Box<dyn error::Error>> {
        Ok(PathBuf::from("temp"))
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
