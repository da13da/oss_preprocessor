use clap::{parser, Parser, ValueEnum};
use dotenv::dotenv;
use std::borrow::BorrowMut;
use std::env;
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

    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN is not set in .env");
    println!("github_token: {:?}", github_token);

    let lock_file_parse_client = parsers::LockFileParseClient::new(args.input)
        .unwrap();
    let mut packages = lock_file_parse_client.parse().unwrap();

    let pypi_client = external_apis::pypi::PypiClient::new();
    for package in &mut packages {
        let package_detail = pypi_client
            .fetch_package_detail(package.name.as_str())
            .await;
        match package_detail {
            Ok(package_detail) => {
                package.latest_version = package_detail.latest_version();
                package.homepage = package_detail.extract_git_url();
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
    }

    let github_client = external_apis::github::GithubClient::new(github_token);
    for package in &packages {
        let (owner, repo) = match package.extract_owner_repo() {
            Some((owner, repo)) => (owner, repo),
            None => {
                println!("Invalid GitHub URL");
                continue;
            }
        };

        let latest_version = match &package.latest_version {
            Some(latest_version) => latest_version,
            None => {
                println!("Invalid latest_version");
                continue;
            }
        };

        if latest_version == &package.current_version {
            println!("{:?} is already latest version", package.name);
            continue;
        };

        let tags = github_client
            .fetch_tags(
                owner.as_str(),
                repo.as_str(),
            )
            .await
            .unwrap();

        let latest_tag = tags.iter().find(| &tag | tag.name == *latest_version);
        let current_tag = tags.iter().find(| &tag | tag.name == package.current_version);

        if let (Some(latest_tag), Some(current_tag)) = (latest_tag, current_tag) {
            let compare_data = github_client
                .fetch_latest_to_current_changes(
                    owner.as_str(),
                    repo.as_str(),
                    current_tag.commit.sha.as_str(),
                    latest_tag.commit.sha.as_str(),
                ).await;

            // println!("{:?}", compare_data);
        } else {
            println!("Failed to extract {:?} tag: latest={:?} current={:?}", package.name, latest_version, package.current_version);
            continue
        }

    }
}
