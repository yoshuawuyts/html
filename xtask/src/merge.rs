use crate::{lookup_json_dir, persist_json, Result};
use html_bindgen::parse::{ParsedElement, ParsedInterface};

pub fn merge() -> Result<()> {
    eprintln!("task: parse elements");
    let elements = lookup_json_dir::<ParsedElement>(crate::PARSED_ELEMENTS_PATH)?;
    let interfaces = lookup_json_dir::<ParsedInterface>(crate::PARSED_WEBIDLS_PATH)?;
    let nodes = html_bindgen::merge::merge(elements, interfaces)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_json(nodes, crate::MERGED_ELEMENTS_PATH)?;
    Ok(())
}
