use std::fs;

use async_std::io::WriteExt;
use structopt::StructOpt;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

/// Tooling for `yosh.is`
#[derive(StructOpt)]
enum Opt {
    /// Generate source code from static sources
    Generate,
    /// Scrape the WebIDL definitions
    Scrape,
    /// Retrieve the latest copy of the HTML standard
    Fetch,
}

#[async_std::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Generate => generate()?,
        Opt::Scrape => scrape()?,
        Opt::Fetch => fetch().await?,
    }
    Ok(())
}

fn generate() -> Result<()> {
    let idl_path = std::env::current_dir()?.join("resources/webidls");

    // generate IDL files
    let database = html_bindgen::parse(&idl_path)?;
    let path = std::env::current_dir()?.join("resources/nodes");
    fs::create_dir_all(&path)?;
    for def in database {
        let path = path.join(format!("{}.json", def.tag_name));

        let s = serde_json::to_string_pretty(&def)?;
        std::fs::write(path, s.to_string().as_bytes())?;
    }

    // generate bindings
    let s = html_bindgen::generate_html(&idl_path)?;

    // write
    let path = std::env::current_dir()?.join("crates/html-sys/src/lib.rs");
    std::fs::write(path, s.as_bytes())?;
    Ok(())
}

async fn fetch() -> Result<()> {
    let target_name = "resources/html-standard/index.html";
    let url = "https://html.spec.whatwg.org";
    eprintln!("fetching: {url}");
    let body = surf::get(url).recv_string().await?;
    let mut target = async_std::fs::File::create(target_name).await?;
    target.write_all(body.as_bytes()).await?;
    eprintln!("updated: {target_name}");
    Ok(())
}

fn scrape() -> Result<()> {
    let path = std::env::current_dir()?.join("resources/html-standard/index.html");
    let spec = fs::read_to_string(path)?;
    html_bindgen::parse_spec(spec)?;
    Ok(())
}
