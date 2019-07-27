use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short, long)]
    ///Pass when decompressing
    pub decompress: bool,

    ///Defines type of compression:`lzw` or `huff`
    pub compressor: String,

    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short, long)]
    pub output: Option<String>,
}