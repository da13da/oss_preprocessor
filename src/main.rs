use clap::{parser, Parser, ValueEnum};
use std::borrow::BorrowMut;
use std::path::PathBuf;

mod entities;
mod external_apis;
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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("input = {:?}, format = {:?}", args.input, args.format);

    let lock_file_parse_client = parsers::LockFileParseClient::new(args.input).unwrap();
    let mut packages = lock_file_parse_client.parse().unwrap();

    let pypi_client = external_apis::pypi::PypiClient::new();
    for mut package in packages {
        let package_detail = pypi_client.get_package_detail(package.name.as_str()).await;
        match package_detail {
            Ok(package_detail) => {
                package.latest_version = package_detail.latest_version();
                package.homepage = package_detail.extract_git_url();
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        println!("{:?}", package);
    }
}
