use anyhow::{Ok, Result};
use clap::{command, Parser};
use colored::Colorize;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    reference: String,
    text: String,
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
    let response = get_response(&args.book, &args.location).await?;

    // Make the heading bold
    println!("{}\n{}", response.reference.bold(), response.text);

    Ok(())
}

async fn get_response(book: &str, location: &str) -> Result<Response> {
    let client = Client::new();

    // Use https://bible-api.com to get the passage
    let body = client
        .get(format!(
            "https://bible-api.com/{book}{location}?verse_numbers=true"
        ))
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(body)
}
