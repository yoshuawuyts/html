use crate::{lookup_json_dir, persist_json, Result};
use html_bindgen::parse::{
    ParsedAriaElement, ParsedAriaProperty, ParsedAriaRole, ParsedElement, ParsedInterface,
};

pub fn merge() -> Result<()> {
    eprintln!("task: parse elements");
    let elements = lookup_json_dir::<ParsedElement>(crate::PARSED_ELEMENTS_PATH)?;
    let interfaces = lookup_json_dir::<ParsedInterface>(crate::PARSED_WEBIDLS_PATH)?;
    let aria_elements = lookup_json_dir::<ParsedAriaElement>(crate::PARSED_ARIA_ELEMENTS_PATH)?;
    let aria_roles = lookup_json_dir::<ParsedAriaRole>(crate::PARSED_ARIA_ROLES_PATH)?;
    let aria_properties =
        lookup_json_dir::<ParsedAriaProperty>(crate::PARSED_ARIA_PROPERTIES_PATH)?;
    let nodes = html_bindgen::merge::merge(
        elements,
        interfaces,
        aria_elements,
        aria_roles,
        aria_properties,
    )?
    .into_iter()
    .map(|n| (n.tag_name.clone(), n));
    persist_json(nodes, crate::MERGED_ELEMENTS_PATH)?;
    Ok(())
}
