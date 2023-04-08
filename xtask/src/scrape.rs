use crate::{persist_nodes, Result};
use std::{env::current_dir, fs};

pub fn scrape_elements() -> Result<()> {
    eprintln!("task: scrape elements");
    let spec = fs::read_to_string(current_dir()?.join(crate::HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_elements(spec)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_nodes(nodes, crate::SCRAPED_ELEMENTS_PATH)?;
    Ok(())
}

pub fn scrape_webidls() -> Result<()> {
    eprintln!("task: scrape webidls");
    let spec = fs::read_to_string(current_dir()?.join(crate::HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_webidls(spec)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_nodes(nodes, crate::SCRAPED_WEBIDLS_PATH)?;
    Ok(())
}
