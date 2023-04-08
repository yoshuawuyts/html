use std::{env::current_dir, fs};
use structopt::StructOpt;

mod fetch;
mod generate;
mod parse;
mod scrape;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

const HTML_STANDARD_URL: &str = "https://html.spec.whatwg.org";
const HTML_STANDARD_PATH: &str = "resources/standards/html.html";
const ARIA_STANDARD_URL: &str = "https://w3c.github.io/html-aria/";
const ARIA_STANDARD_PATH: &str = "resources/standards/aria.html";
const SCRAPED_ELEMENTS_PATH: &str = "resources/scraped/elements";
const SCRAPED_WEBIDLS_PATH: &str = "resources/scraped/webidls";
const PARSED_ELEMENTS_PATH: &str = "resources/parsed/elements";
const HTML_SYS_CRATE_PATH: &str = "crates/html-sys/src";
const HTML_CRATE_ELEMENTS_PATH: &str = "crates/html/src/generated";
const MANUAL_PATH: &str = "resources/manual";

/// Tooling for the Rust `html` crate
#[derive(StructOpt)]
enum Opt {
    /// Fetch, scrape, parse, and generate bindings
    All,
    /// Fetch the latest copies of the HTML standards
    Fetch,
    /// Scrape the raw standards into structured data
    Scrape,
    /// Parse the structured standards data into normalized form
    Parse,
    /// Generate Rust source code from the parsed data
    Generate,
}

#[async_std::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::All => all().await?,
        Opt::Generate => {
            generate::generate_sys()?;
            generate::generate_html()?;
        }
        Opt::Scrape => {
            scrape::scrape_elements()?;
            scrape::scrape_webidls()?;
        }
        Opt::Parse => parse::parse()?,
        Opt::Fetch => fetch::fetch().await?,
    }
    Ok(())
}

async fn all() -> Result<()> {
    fetch::fetch().await?;
    scrape::scrape_elements()?;
    scrape::scrape_webidls()?;
    generate::generate_sys()?;
    generate::generate_html()?;
    Ok(())
}

fn lookup_nodes<T: serde::de::DeserializeOwned>(
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

fn lookup_file<T: serde::de::DeserializeOwned>(path: &str, name: &str) -> Result<T> {
    let path = current_dir()?.join(path).join(format!("{name}.json"));
    let s = fs::read_to_string(&path)?;
    let parsed = serde_json::from_str(&s)?;
    Ok(parsed)
}

fn persist_nodes<T: serde::Serialize>(
    nodes: impl Iterator<Item = (String, T)>,
    dest: &str,
) -> Result<()> {
    let path = current_dir()?.join(dest);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    for (name, node) in nodes {
        let path = path.join(format!("{}.json", name));
        eprintln!("writing: {dest}/{}.json", name);
        let s = serde_json::to_string_pretty(&node)?;
        std::fs::write(path, s.to_string().as_bytes())?;
    }
    Ok(())
}
