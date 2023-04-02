use anyhow::{Context, Ok, Result};
use clap::{command, Parser};
use colored::Colorize;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Query {
    passages: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The book of the bible to view
    book: String,
    /// The chapter and verse number for the verse
    location: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Access the ESV API to get the Bible verses
    let verse = get_verse(&args.book, &args.location).await?;

    // Seperate the heading and the content
    let seperated: Vec<&str> = verse.passages[0].split("\n\n").collect();

    // Make the heading bold
    println!("{}\n{}", seperated[0].bold(), seperated[1]);

    Ok(())
}

async fn get_verse(book: &str, location: &str) -> Result<Query> {
    let client = Client::new();

    // I do not want to post the ESV api token to GitHub
    let token = fs::read_to_string(".token").context("Failed to read token file")?;

    // Use the ESV api to get the passage
    let body = client
        .get(format!(
            "https://api.esv.org/v3/passage/text/?q={book}+{location}"
        ))
        .header(AUTHORIZATION, format!("Token {token}"))
        .send()
        .await?
        .json::<Query>()
        .await?;

    Ok(body)
}
