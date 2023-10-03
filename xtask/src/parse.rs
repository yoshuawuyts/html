use crate::{lookup_dir, lookup_json_dir, persist_json, Result};
use html_bindgen::scrape::{
    ScrapedAriaElement, ScrapedAriaProperty, ScrapedAriaRole, ScrapedElement,
};

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

pub fn parse_aria_elements() -> Result<()> {
    eprintln!("task: parse aria elements");
    let iter = lookup_json_dir::<ScrapedAriaElement>(crate::SCRAPED_HTML_ARIA_PATH)?;
    let nodes = html_bindgen::parse::parse_aria_elements(iter)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_json(nodes, crate::PARSED_ARIA_ELEMENTS_PATH)?;
    Ok(())
}

pub fn parse_aria_roles() -> Result<()> {
    eprintln!("task: parse aria roles");
    let iter = lookup_json_dir::<ScrapedAriaRole>(crate::SCRAPED_ARIA_ROLES_PATH)?;
    let nodes = html_bindgen::parse::parse_aria_roles(iter)?
        .into_iter()
        .map(|n| (n.name.clone(), n));
    persist_json(nodes, crate::PARSED_ARIA_ROLES_PATH)?;
    Ok(())
}

pub fn parse_aria_properties() -> Result<()> {
    eprintln!("task: parse aria properties");
    let iter = lookup_json_dir::<ScrapedAriaProperty>(crate::SCRAPED_ARIA_PROPERTIES_PATH)?;
    let nodes = html_bindgen::parse::parse_aria_properties(iter)?
        .into_iter()
        .map(|n| (n.name.clone(), n));
    persist_json(nodes, crate::PARSED_ARIA_PROPERTIES_PATH)?;
    Ok(())
}
