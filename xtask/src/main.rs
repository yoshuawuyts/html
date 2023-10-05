use std::{env::current_dir, fs, path::PathBuf};
use structopt::StructOpt;

mod debug;
mod fetch;
mod generate;
mod merge;
mod parse;
mod scrape;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

const HTML_STANDARD_URL: &str = "https://html.spec.whatwg.org";
const HTML_STANDARD_PATH: &str = "resources/standards/html.html";
const ARIA_STANDARD_URL: &str = "https://w3c.github.io/aria/";
const HTML_ARIA_STANDARD_URL: &str = "https://w3c.github.io/html-aria/";
const ARIA_STANDARD_PATH: &str = "resources/standards/aria.html";
const HTML_ARIA_STANDARD_PATH: &str = "resources/standards/html-aria.html";
const SCRAPED_ELEMENTS_PATH: &str = "resources/scraped/elements";
const SCRAPED_WEBIDLS_PATH: &str = "resources/scraped/webidls";
const SCRAPED_ARIA_ROLES_PATH: &str = "resources/scraped/aria/roles";
const SCRAPED_ARIA_PROPERTIES_PATH: &str = "resources/scraped/aria/properties";
const SCRAPED_HTML_ARIA_PATH: &str = "resources/scraped/aria/elements";
const PARSED_ELEMENTS_PATH: &str = "resources/parsed/elements";
const PARSED_WEBIDLS_PATH: &str = "resources/parsed/webidls";
const PARSED_ARIA_ELEMENTS_PATH: &str = "resources/parsed/aria/elements";
const PARSED_ARIA_ROLES_PATH: &str = "resources/parsed/aria/roles";
const PARSED_ARIA_PROPERTIES_PATH: &str = "resources/parsed/aria/properties";
const MERGED_ELEMENTS_PATH: &str = "resources/merged/elements";
const HTML_SYS_CRATE_PATH: &str = "crates/html-sys/src";
const HTML_CRATE_ELEMENTS_PATH: &str = "crates/html/src/generated";
const MANUAL_PATH: &str = "resources/manual";

/// Tooling for the Rust `html` crate
#[derive(StructOpt)]
enum Opt {
    /// Fetch, scrape, parse, and generate bindings
    All,
    /// Run everything except `fetch`
    NoFetch,
    /// Fetch the latest copies of the HTML standards
    Fetch,
    /// Scrape the raw standards into structured data
    Scrape,
    /// Parse the structured standards data into normalized forms
    Parse,
    /// Merge the normalized data into a single structure
    Merge,
    /// Generate Rust source code from the unified data
    Generate,
    /// Debug the relationship between two nodes
    Debug { parent: String, child: String },
}

#[async_std::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::All => all().await?,
        Opt::Fetch => fetch::fetch().await?,
        Opt::Scrape => {
            scrape::scrape_elements()?;
            scrape::scrape_webidls()?;
            scrape::scrape_aria()?;
            scrape::scrape_html_aria()?;
        }
        Opt::Parse => {
            parse::parse_webidls()?;
            parse::parse_elements()?;
            parse::parse_aria_elements()?;
            parse::parse_aria_roles()?;
            parse::parse_aria_properties()?;
        }
        Opt::Generate => {
            generate::generate_sys()?;
            generate::generate_html()?;
        }
        Opt::Merge => {
            merge::merge()?;
        }
        Opt::NoFetch => {
            no_fetch().await?;
        }
        Opt::Debug { parent, child } => debug::debug(parent, child).await?,
    }
    Ok(())
}

async fn all() -> Result<()> {
    fetch::fetch().await?;
    no_fetch().await?;
    Ok(())
}

async fn no_fetch() -> Result<()> {
    scrape::scrape_elements()?;
    scrape::scrape_webidls()?;
    scrape::scrape_aria()?;
    scrape::scrape_html_aria()?;
    parse::parse_elements()?;
    parse::parse_webidls()?;
    parse::parse_aria_elements()?;
    parse::parse_aria_roles()?;
    parse::parse_aria_properties()?;
    merge::merge()?;
    generate::generate_sys()?;
    generate::generate_html()?;
    Ok(())
}

fn lookup_json_dir<T: serde::de::DeserializeOwned>(
    src: &str,
) -> Result<impl Iterator<Item = Result<T>>> {
    let path = current_dir()?.join(src);
    let iter = fs::read_dir(path)?.into_iter().map(|path| -> Result<T> {
        let s = fs::read_to_string(path?.path())?;
        let parsed = serde_json::from_str(&s)?;
        Ok(parsed)
    });
    Ok(iter)
}

fn lookup_json_file<T: serde::de::DeserializeOwned>(path: &str, name: &str) -> Result<T> {
    let path = current_dir()?.join(path).join(format!("{name}.json"));
    let s = fs::read_to_string(&path)?;
    let parsed = serde_json::from_str(&s)?;
    Ok(parsed)
}

fn lookup_dir(src: &str) -> Result<impl Iterator<Item = Result<(String, PathBuf)>>> {
    let path = current_dir()?.join(src);
    let iter = fs::read_dir(path)?.into_iter().map(|path| {
        let path = path?.path();
        Ok((fs::read_to_string(path.clone())?, path))
    });
    Ok(iter)
}

fn persist_json<T: serde::Serialize>(
    files: impl Iterator<Item = (String, T)>,
    directory: &str,
) -> Result<()> {
    let path = current_dir()?.join(directory);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    for (name, s) in files {
        let path = path.join(format!("{name}.json"));
        eprintln!("writing: {directory}/{name}.json");
        let s = serde_json::to_string_pretty(&s)?;
        std::fs::write(path, s.to_string().as_bytes())?;
    }
    Ok(())
}

fn persist_files(
    files: impl Iterator<Item = (String, String)>,
    directory: &str,
    file_extension: &str,
) -> Result<()> {
    let path = current_dir()?.join(directory);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    for (name, s) in files {
        let path = path.join(format!("{name}.{file_extension}"));
        eprintln!("writing: {directory}/{name}.{file_extension}");
        std::fs::write(path, s.to_string().as_bytes())?;
    }
    Ok(())
}
