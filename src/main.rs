use clap::{parser, Parser, ValueEnum};
use std::path::PathBuf;

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

    let lock_file_parse_client = parsers::LockFileParseClient::new(args.input).unwrap();
    let packages = lock_file_parse_client.parse().unwrap();
    println!("{:?}", packages);
}
