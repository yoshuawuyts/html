use crate::{persist_files, persist_json, Result};
use std::{env::current_dir, fs};

pub fn scrape_elements() -> Result<()> {
    eprintln!("task: scrape elements");
    let spec = fs::read_to_string(current_dir()?.join(crate::HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_elements(spec)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_json(nodes, crate::SCRAPED_ELEMENTS_PATH)?;
    Ok(())
}

pub fn scrape_webidls() -> Result<()> {
    eprintln!("task: scrape webidls");
    let spec = fs::read_to_string(current_dir()?.join(crate::HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_webidls(spec)?
        .into_iter()
        .map(|n| (n.name, n.idl));
    persist_files(nodes, crate::SCRAPED_WEBIDLS_PATH, "webidl")?;
    Ok(())
}

pub fn scrape_aria() -> Result<()> {
    eprintln!("task: scrape aria");
    let spec = fs::read_to_string(current_dir()?.join(crate::ARIA_STANDARD_PATH))?;
    let (roles, properties) = html_bindgen::scrape::scrape_aria(spec)?;
    let role_nodes = roles.into_iter().map(|n| (n.name.clone(), n));
    persist_json(role_nodes, crate::SCRAPED_ARIA_ROLES_PATH)?;
    let property_nodes = properties.into_iter().map(|n| (n.name.clone(), n));
    persist_json(property_nodes, crate::SCRAPED_ARIA_PROPERTIES_PATH)?;
    Ok(())
}

pub fn scrape_html_aria() -> Result<()> {
    eprintln!("task: scrape html aria");
    let spec = fs::read_to_string(current_dir()?.join(crate::HTML_ARIA_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_html_aria(spec)?;
    let nodes = nodes.into_iter().map(|n| (n.id.clone(), n));
    persist_json(nodes, crate::SCRAPED_HTML_ARIA_PATH)?;
    Ok(())
}
