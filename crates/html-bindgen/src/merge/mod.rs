//! Unify all the parsed sources into a single, final source.

use std::collections::HashMap;

use crate::parse::{Attribute, Category, ParsedElement, ParsedInterface};
use crate::Result;
use serde::{Deserialize, Serialize};

/// The final source of truth we used to generate code from.
///
/// Created by combining all of the previously parsed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedElement {
    pub tag_name: String,
    pub struct_name: String,
    pub submodule_name: String,
    pub mdn_link: String,
    pub has_global_attributes: bool,
    pub has_closing_tag: bool,
    pub attributes: Vec<Attribute>,
    pub dom_interface: String,
    pub content_categories: Vec<Category>,
    pub permitted_content: Vec<Category>,
    pub permitted_parents: Vec<Category>,
}

pub fn merge(
    parsed_elements: impl Iterator<Item = Result<ParsedElement>>,
    parsed_interfaces: impl Iterator<Item = Result<ParsedInterface>>,
) -> Result<Vec<NormalizedElement>> {
    let mut output = vec![];
    let mut elements = HashMap::new();
    for el in parsed_elements {
        let el = el?;
        let key = el.tag_name.clone();
        elements.insert(key, el);
    }

    let mut interfaces = HashMap::new();
    for interface in parsed_interfaces {
        let interface = interface?;
        let key = interface.name.clone();
        interfaces.insert(key, interface);
    }

    let by_content_type = normalize_content_types(&elements);
    let children_map = normalize_children(&elements, &by_content_type);
    dbg!(children_map);
    Ok(output)
}

/// Sort all elements into their respective content type.
fn normalize_content_types(
    elements: &HashMap<String, ParsedElement>,
) -> HashMap<Category, Vec<String>> {
    let mut output = HashMap::new();
    for (name, el) in elements {
        for cat in &el.content_categories {
            let vec: &mut Vec<_> = output.entry(cat.clone()).or_default();
            vec.push(name.clone());
        }
    }
    output
}

/// Which element can have which children?
fn normalize_children(
    elements: &HashMap<String, ParsedElement>,
    by_content_type: &HashMap<Category, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut output = HashMap::with_capacity(elements.len());
    for (name, el) in elements {
        for category in &el.permitted_parents {
            for parent in by_content_type.get(&category).unwrap() {
                let vec: &mut Vec<_> = output.entry(parent.clone()).or_default();
                vec.push(name.clone());
            }
        }
    }
    output
}
