use std::borrow::BorrowMut;
use clap::{parser, Parser, ValueEnum};
use std::path::PathBuf;
use rustpython::vm::stdlib::builtins::print;

mod entities;
mod parsers;
mod external_apis;

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
        let package_details = pypi_client.get_package_details(package.name.as_str()).await;
        let latest_version = match package_details {
            Ok(package_details) => package_details.latest_version(),
            Err(err) => {
                eprintln!("Error: {}", err);
                None
            },
        };
        package.latest_version = latest_version;
        println!("{:?}", package);
    }
}
