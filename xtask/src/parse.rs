use crate::{lookup_nodes, persist_json, Result};
use html_bindgen::scrape::ScrapedElement;

pub fn parse() -> Result<()> {
    eprintln!("task: parse");
    let iter = lookup_nodes::<ScrapedElement>(crate::SCRAPED_ELEMENTS_PATH)?;
    let nodes = html_bindgen::parse::parse(iter)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_json(nodes, crate::PARSED_ELEMENTS_PATH)?;
    Ok(())
}
