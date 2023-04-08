use crate::{utils::extract_webidl_name, Result};
use serde::{Deserialize, Serialize};

/// The raw values extracted from the HTML spec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedInterface {
    pub name: String,
    pub idl: String,
}

/// Parse the WhatWG HTML standards document.
///
/// # Design
///
/// The entire HTML spec is a flat document with little hierarchy. we first need to find
/// the metadata section labeled by `.element`. Then we need to track back through the
/// siblings to find the first `h4` node. That will contain the title of the elements.
///
/// Once we have the title, we can inspect the `.element` node properly. This is a nested
/// table containing strings. We then parse these strings into a structured representation.
pub fn scrape_webidls(spec: String) -> Result<Vec<ScrapedInterface>> {
    let document = scraper::Html::parse_document(&spec);
    let selector = scraper::Selector::parse(".idl").unwrap();

    let mut specs = vec![];
    for element in document.select(&selector).into_iter() {
        let idl = element.text().map(|t| t.to_owned()).collect::<String>();
        let name = match extract_webidl_name(&idl) {
            Some(name) => name,
            None => continue,
        };
        specs.push(ScrapedInterface { name, idl });
    }
    Ok(specs)
}
