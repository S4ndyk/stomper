use libstomper::{huffman::Huffman, lzw::LZW, Compressor};
use std::error;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use structopt::StructOpt;

pub fn run(args: Args) -> Result<(), Box<dyn error::Error>> {
    let input = BufReader::new(File::open(args.input)?);
    let out_filename = args.output.unwrap_or(PathBuf::from("output.stmpd"));
    let output = BufWriter::new(File::create(out_filename)?);
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
