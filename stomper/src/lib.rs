//! Frontend for stomper compression tool. See [libstomper](https://docs.rs/libstomper/0.1.0/libstomper/) for implemented algorithms
//!
use libstomper::{huffman::Huffman, lzw::LZW, Compressor};
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::path::PathBuf;
use structopt::StructOpt;

/// Runs program
pub fn run(args: Args) -> Result<(), Box<dyn error::Error>> {
    // Choose where to write encoded/decoded data
    // Defaults as output.[compressor].[comp/decomp] if no argrument given
    let mut input = BufReader::new(File::open(&args.input)?);
    let out_filename = if let Some(path) = args.output {
        path
    } else {
        if !args.input.is_file() {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "input is not a file")));
        }
        let mut default = PathBuf::new();
        let compressor = &args.compressor;
        let decompress = if args.decompress {
            "decomp"
        } else {
            "comp"
        };
        default.set_file_name(format!("stomped.{}.{}",compressor, decompress));
        default
    };
    
    let mut output = BufWriter::new(File::create(out_filename)?);
    // Chooses the type of compression based on args.compressor
    // Return error if no match is found
    choose_compression(args.compressor.as_str(), args.decompress, &mut input, &mut output)?;
    Ok(())
}

fn choose_compression<R, W>(comp: &str, decomp: bool, inp: &mut R, out: &mut W) -> Result<(), Box<dyn error::Error>>
    where R: Read + Seek,
          W: Write + Seek 
          {
    match comp {
        "lzw" => match decomp {
            true => LZW::decode(inp, out),
            false => LZW::encode(inp, out),
        },
        "huffman" | "huff" => match decomp{
            true => Huffman::decode(inp, out),
            false => Huffman::encode(inp, out),
        },

        s => {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                format!("{} is not a compression type", s),
            )))
        }
    }?;
    Ok(())
}

#[derive(StructOpt)]
pub struct Args {
    /// Flag for decompressing
    #[structopt(short, long)]
    pub decompress: bool,
    /// Supported compressors: lzw, huffman
    pub compressor: String,
    /// Path to file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Optional name of output file
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
}
