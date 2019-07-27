pub mod args;
use std::error;
use std::fs::File;
mod huffman;
mod lzw;
use args::Args;
use huffman::Huffman;
use lzw::LZW;

pub fn run(args: &Args) -> Result<(), Box<dyn error::Error>> {
    let input = File::open(&args.input)?;
    let output = &args.output;

    //This is a very ugly temporary solution
    match args.compressor.as_str() {
        "lzw" => {
            if args.decompress {
                LZW::decompress(&input, output)
            } else {
                LZW::compress(&input, output)
            }
        }
        "huff" | "huffman" => {
            if args.decompress {
                Huffman::decompress(&input, output)
            } else {
                Huffman::compress(&input, output)
            }
        }
        _ => eprintln!("Invalid compression type!"),
    };
    Ok(())
}

pub trait Compressor {
    fn compress(input: &File, output: &Option<String>);
    fn decompress(input: &File, output: &Option<String>);
}
