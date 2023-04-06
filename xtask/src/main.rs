use std::{env::current_dir, fs};

use async_std::io::WriteExt;
use structopt::StructOpt;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

const HTML_STANDARD_URL: &str = "https://html.spec.whatwg.org";
const HTML_STANDARD_PATH: &str = "resources/standards/html.html";
const SCRAPED_NODES_PATH: &str = "resources/scraped";
const PARSED_NODES_PATH: &str = "resources/parsed";
const IDL_PATH: &str = "resources/webidls";

/// Tooling for `yosh.is`
#[derive(StructOpt)]
enum Opt {
    /// Fetch, parse, and generate bindings
    All,
    /// Generate source code from static sources
    Generate,
    /// Parse the WebIDL definitions
    Scrape,
    /// Retrieve the latest copy of the HTML standard
    Fetch,
}

#[async_std::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::All => all().await?,
        Opt::Generate => generate()?,
        Opt::Scrape => scrape()?,
        Opt::Fetch => fetch().await?,
    }
    Ok(())
}

async fn all() -> Result<()> {
    fetch().await?;
    scrape()?;
    generate()?;
    Ok(())
}

async fn fetch() -> Result<()> {
    eprintln!("fetching: {HTML_STANDARD_URL}");
    let body = surf::get(HTML_STANDARD_URL).recv_string().await?;
    let mut target = async_std::fs::File::create(HTML_STANDARD_PATH).await?;
    target.write_all(body.as_bytes()).await?;
    eprintln!("updated: {HTML_STANDARD_PATH}");
    Ok(())
}

fn scrape() -> Result<()> {
    let spec = fs::read_to_string(current_dir()?.join(HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape_spec(spec)?;

    let path = current_dir()?.join(SCRAPED_NODES_PATH);
    fs::create_dir_all(&path)?;
    for node in nodes {
        let path = path.join(format!("{}.json", node.tag_name));
        eprintln!("writing: {SCRAPED_NODES_PATH}/{}.json", node.tag_name);
        let s = serde_json::to_string_pretty(&node)?;
        std::fs::write(path, s.to_string().as_bytes())?;
    }
    Ok(())
}

fn generate() -> Result<()> {
    let idl_path = current_dir()?.join(IDL_PATH);

    // generate IDL files
    let database = html_bindgen::parse(&idl_path)?;
    let path = current_dir()?.join(PARSED_NODES_PATH);
    fs::create_dir_all(&path)?;
    for def in database {
        let path = path.join(format!("{}.json", def.tag_name));

        let s = serde_json::to_string_pretty(&def)?;
        std::fs::write(path, s.to_string().as_bytes())?;
    }

    // generate bindings
    let s = html_bindgen::generate_html(&idl_path)?;

    // write
    let path = current_dir()?.join("crates/html-sys/src/lib.rs");
    std::fs::write(path, s.as_bytes())?;
    Ok(())
}
