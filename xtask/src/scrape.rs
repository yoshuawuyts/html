use crate::{persist_nodes, Result};
use std::{env::current_dir, fs};

pub fn scrape() -> Result<()> {
    let spec = fs::read_to_string(current_dir()?.join(crate::HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_spec(spec)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_nodes(nodes, crate::SCRAPED_ELEMENTS_PATH)?;
    Ok(())
}
