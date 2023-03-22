use anyhow;
use clap::Parser;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use ureq;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let html = fetch(&cli.url, &cli.user_agent)?;
    let mut data = parse_td(&html);

    if cli.integers {
        data.retain(|x| !is_int(&x))
    }

    if cli.ascii {
        data.retain(|x| x.is_ascii())
    }

    if cli.lowercase {
        data.iter_mut().for_each(|x| x.make_ascii_lowercase())
    }

    if cli.spaces {
        data.iter_mut().for_each(|x| x.retain(|y| !y.is_whitespace()))
    }

    if let Some(min) = cli.min {
        data.retain(|x| x.len() >= min)
    }

    if let Some(max) = cli.max {
        data.retain(|x| x.len() <= max)
    }

    if let Some(output) = cli.output.as_deref() {
        fs::write(output, data.into_iter().collect::<Vec<String>>().join("\n"))?;
    } else {
        for td in data {
            println!("{td}");
        }
    }

    Ok(())
}

fn is_int(word: &str) -> bool {
    word.chars().all(|x| x.is_ascii_digit())
}

fn parse_td(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("td").unwrap();
    let data: HashSet<String> = document
        .select(&selector)
        .map(|x| x.text())
        .flatten()
        .map(|x| x.trim())
        .map(|x| x.to_owned())
        .collect();
    Vec::from_iter(data)
}

fn fetch(url: &str, user_agent: &str) -> Result<String, ureq::Error> {
    let body = ureq::get(url)
        .set("User-Agent", user_agent)
        .call()?
        .into_string()?;

    Ok(body)
}

#[derive(Parser)]
#[command(author = "Pete Buffon <pabuffon@gmail.com>")]
#[command(about = "🦀 tdnab 🦀, HTML table <td> scraper")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Url to scrape
    url: String,

    /// Outfile for wordlist
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,


    /// Exclude cells with non ASCII characters
    #[arg(short, long)]
    ascii: bool,

    /// Exclude integer only cells
    #[arg(short, long)]
    integers: bool,

    /// Transform characters to lowercase
    #[arg(short, long)]
    lowercase: bool,

    /// Maximum length of cells to keep
    #[arg(short = 'M', long, value_name = "MAX")]
    max: Option<usize>,

    /// Minimum length of cells to keep
    #[arg(short, long, value_name = "MIN")]
    min: Option<usize>,

    /// Remove spaces
    #[arg(short, long)]
    spaces: bool,

    /// Custom user-agent header
    #[arg(short, long, default_value_t = String::from("tdnab/") + env!("CARGO_PKG_VERSION"))]
    user_agent: String,
}