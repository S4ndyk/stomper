//! Tool for reading bits one by one from stream

use std::io::Read;

const BUFFER_SIZE: usize = 128;

pub struct BitReader<R: Read> {
    reader: R,
    buffer: [u8; BUFFER_SIZE],
    end: usize,
    bytecount: usize,
    bitpos: usize,
}

impl <R: Read> BitReader<R> {

    /// Returns new bitreader and takes ownership of given reader
    pub fn new(reader: R) -> Self {
        let mut bitreader = BitReader {
            reader,
            buffer: [0; BUFFER_SIZE],
            end: 0,
            bytecount: 0,
            bitpos: 0,
        };
        bitreader.read_to_buffer();
        bitreader
    } 

    /// Return next bit in the stream and None reached end of file. True represent 1 and false 0
    pub fn next_bit(&mut self) -> Option<bool> {
        // Read more bytes if buffer is full
        if self.bytecount > BUFFER_SIZE - 1 {
            self.read_to_buffer();
        }

        // Returns none if reached EOF
        if self.bytecount == self.end  && self.read_to_buffer() == 0 {
            return None
        };

        let next = self.buffer[self.bytecount];
        let mask = 1 << self.bitpos;
        self.bitpos += 1;
        if self.bitpos > 7 {
            self.bytecount += 1;
            self.bitpos = 0;
        }
        if next & mask != 0 {
            Some(true)
        } else {
            Some(false)
        }
    }    

    /// Reads bytes from the underlying reader to buffer
    fn read_to_buffer(&mut self) -> usize {
        self.end = self.reader.read(&mut self.buffer).expect("Could not read to buffer");
        self.bytecount = 0;
        self.bitpos = 0;
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::BitReader;
    #[test]
    fn reader_reads_correctly_no1() {
        let mut v = Vec::new();
        v.push(0b10110011);
        let mut reader = BitReader::new(v.as_slice());
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), None);
    }

    #[test]
    fn reader_reads_correctly_no2() {
        let mut v = Vec::new();
        v.push(0b10110011);
        v.push(0b11110000);
        v.push(0b00110011);
        let mut reader = BitReader::new(v.as_slice());
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(true));
        

        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        
        
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(true));
        assert_eq!(reader.next_bit(), Some(false));
        assert_eq!(reader.next_bit(), Some(false));

        assert_eq!(reader.next_bit(), None) 
    }

    #[test]
    fn reader_reads_till_end() {
        let mut v: Vec<u8> = Vec::new();
        let bytes = 20000; 
        let bits = bytes * 8;
        for _i in 0..bytes {
            v.push(21);
        }
        let mut bits_read = 0;
        let mut reader = BitReader::new(v.as_slice());
        while let Some(_bit) = reader.next_bit() {
            bits_read += 1;
        }
        assert_eq!(bits, bits_read);
    }
}