use crate::{lookup_dir, lookup_json_dir, persist_json, Result};
use html_bindgen::scrape::ScrapedElement;

pub fn parse_elements() -> Result<()> {
    eprintln!("task: parse elements");
    let iter = lookup_json_dir::<ScrapedElement>(crate::SCRAPED_ELEMENTS_PATH)?;
    let nodes = html_bindgen::parse::parse_elements(iter)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_json(nodes, crate::PARSED_ELEMENTS_PATH)?;
    Ok(())
}

pub fn parse_webidls() -> Result<()> {
    eprintln!("task: parse webidls");
    let iter = lookup_dir(crate::SCRAPED_WEBIDLS_PATH)?;
    let nodes = html_bindgen::parse::parse_webidls(iter)?
        .into_iter()
        .map(|n| (n.name.clone(), n));
    persist_json(nodes, crate::PARSED_WEBIDLS_PATH)?;
    Ok(())
}
