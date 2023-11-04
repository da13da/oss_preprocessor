use std::path::PathBuf;
use clap::{Parser, parser, ValueEnum};

mod entities;
mod parsers;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATH")]
    input: PathBuf,

    #[arg(short, long, value_enum, help = "file type")]
    format: Format,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    lock,
    result,
}

fn main() {
    let args = Args::parse();
    println!("input = {:?}, format = {:?}", args.input, args.format);

    let lock_file_parse_client= parsers::LockFileParseClient::new(args.input).unwrap();
    // println!("file={:?}", lock_file_parse_client.lock_file_str);

    let packages = lock_file_parse_client.parse().unwrap();
    println!("{:?}", packages[0]);
}
