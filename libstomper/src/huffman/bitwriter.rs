use std::io::Write;

const BUFFER_SIZE: usize = 128;

pub struct BitWriter<W: Write>{
    writer: W,
    buffer: [u8; BUFFER_SIZE],
    bytecount: usize,
    bitpos: usize,
}

impl <W: Write> BitWriter<W> {
    fn new(writer: W) -> Self {
        BitWriter {
            writer,
            buffer: [0; BUFFER_SIZE],
            bytecount: 0,
            bitpos: 0,
        }
    } 

    fn write_bit(&mut self, set_bit: bool) {
        if set_bit {
            let mask = 1 << self.bitpos;
            let byte = self.buffer[self.bytecount];
            self.buffer[self.bytecount] = byte | mask;
        }
        self.bitpos += 1;
        if self.bitpos > 7 {
            self.bitpos = 0;
            self.bytecount += 1;
        }

        if self.bytecount > BUFFER_SIZE - 1 {
            self.flush();
        }
    } 

    fn flush(&mut self) {
        self.writer.write_all(&self.buffer).expect("Could not write to all to writer");
        self.bitpos = 0;
        self.bytecount = 0;
        self.buffer = [0; BUFFER_SIZE];
    }

    pub fn write_string(&mut self, s: String) {
        for c in s.bytes() {
            if c as char == '1' {
                self.write_bit(true);
            }
            if c as char == '0' {
                self.write_bit(false);
            }
        }
    }

    /// Flushes and returns inner writer 
    fn to_inner(mut self) -> W {
        self.flush();
        self.writer
    }

}

#[cfg(test)]
mod tests {
    use super::BitWriter;

    #[test]
    fn writer_writes_correctly_no1() {
        let s = String::from("100101");
        let buf: Vec<u8> = Vec::new();
        let mut writer = BitWriter::new(buf);
        writer.write_string(s);
        let buf = writer.to_inner();
        assert_eq!(buf[0], 0b101001);
    }

    #[test]
    fn writer_writes_correctly_no2() {
        let s = String::from("111001011101");
        let buf: Vec<u8> = Vec::new();
        let mut writer = BitWriter::new(buf);
        writer.write_string(s);
        let buf = writer.to_inner();
        assert_eq!(buf[0], 0b10100111);
        assert_eq!(buf[1], 0b1011);
    }

    #[test]
    fn writer_writes_correctly_no3() {
        let s1 = String::from("1110");
        let s2 = String::from("01");
        let s3 = String::from("01110");
        let s4 = String::from("1");
        let buf: Vec<u8> = Vec::new();
        let mut writer = BitWriter::new(buf);
        writer.write_string(s1);
        writer.write_string(s2);
        writer.write_string(s3);
        writer.write_string(s4);
        let buf = writer.to_inner();
        assert_eq!(buf[0], 0b10100111);
        assert_eq!(buf[1], 0b1011);
    }

    #[test]
    fn writer_does_not_overflow() {
        let buf: Vec<u8> = Vec::new();
        let mut writer = BitWriter::new(buf);
        for i in 0..3072 {
            if i%3 == 0 {
                writer.write_bit(false);
            } else {
                writer.write_bit(true);
            }
        }
    }
}