//! Frontend for stomper compression tool. See [libstomper](../libstomper/index.html) for implemented algorithms
//!
use libstomper::{huffman::Huffman, lzw::LZW, Compressor};
use std::error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::path::PathBuf;
use structopt::StructOpt;

/// Runs program
pub fn run(args: Args) -> Result<(), Box<dyn error::Error>> {
    let input = BufReader::new(File::open(args.input)?);
    // Choose where to write encoded/decoded data
    // Defaults as output.stmpd if no argrument given
    let out_filename = args.output.unwrap_or(PathBuf::from("output.stmpd"));
    let output = BufWriter::new(File::create(out_filename)?);

    // Chooses the type of compression based on args.compressor
    // Return error if no match is found
    match args.compressor.as_str() {
        "lzw" => match args.decompress {
            true => LZW::decode(input, output),
            false => LZW::encode(input, output),
        },
        "huffman" | "huff" => match args.decompress {
            true => Huffman::decode(input, output),
            false => Huffman::encode(input, output),
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

/// Struct for command-line arguments collected with structopt crate
#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long)]
    pub decompress: bool,

    pub compressor: String,

    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short, long)]
    pub output: Option<PathBuf>,
}
