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
    pub permitted_child_elements: Vec<String>,
}

pub fn merge(
    parsed_elements: impl Iterator<Item = Result<ParsedElement>>,
    parsed_interfaces: impl Iterator<Item = Result<ParsedInterface>>,
) -> Result<Vec<NormalizedElement>> {
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

    let by_content_type = elements_per_content_type(&elements);
    let children_map = children_per_element(&elements, &by_content_type);

    let mut output = vec![];
    for (_, el) in elements.into_iter() {
        let permitted_child_elements = children_map.get(dbg!(&el.tag_name)).unwrap().clone();
        output.push(NormalizedElement {
            tag_name: el.tag_name,
            struct_name: el.struct_name,
            submodule_name: el.submodule_name,
            mdn_link: el.mdn_link,
            has_global_attributes: el.has_global_attributes,
            has_closing_tag: el.has_closing_tag,
            attributes: el.attributes,
            dom_interface: el.dom_interface,
            content_categories: el.content_categories,
            permitted_child_elements,
        })
    }
    Ok(output)
}

/// Sort all elements into their respective content type.
fn elements_per_content_type(
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
fn children_per_element(
    elements: &HashMap<String, ParsedElement>,
    by_content_type: &HashMap<Category, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut output: HashMap<_, _> = elements
        .iter()
        .map(|(name, _)| (name.clone(), vec![]))
        .collect();
    for (_, el) in elements {
        for category in &el.permitted_parents {
            let parents = match by_content_type.get(&category) {
                Some(parent) => parent.clone(),
                None => vec![],
            };

            for parent in parents {
                let vec: &mut Vec<_> = output.entry(parent.clone()).or_default();
                vec.push(el.struct_name.clone());
            }
        }
    }
    output
}
