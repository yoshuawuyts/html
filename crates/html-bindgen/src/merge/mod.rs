//! Unify all the parsed sources into a single, final source.

use std::collections::HashMap;

use crate::parse::{Attribute, Category, ParsedElement, ParsedInterface};
use crate::Result;
use serde::{Deserialize, Serialize};

/// The final source of truth we used to generate code from.
///
/// Created by combining all of the previously parsed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergedElement {
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
) -> Result<Vec<MergedElement>> {
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
    let attributes_map = merge_attributes(&elements, &interfaces);

    let mut output = vec![];
    for (_, el) in elements.into_iter() {
        let permitted_child_elements = children_map.get(&el.tag_name).unwrap().clone();
        let attributes = attributes_map.get(&el.tag_name).unwrap().clone();
        output.push(MergedElement {
            tag_name: el.tag_name,
            struct_name: el.struct_name,
            submodule_name: el.submodule_name,
            mdn_link: el.mdn_link,
            has_global_attributes: el.has_global_attributes,
            has_closing_tag: el.has_closing_tag,
            dom_interface: el.dom_interface,
            content_categories: el.content_categories,
            attributes,
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
    let mut output = elements
        .iter()
        .map(|(name, _)| (name.clone(), vec![]))
        .collect::<HashMap<_, _>>();
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

    output.iter_mut().for_each(|(_, value)| value.sort());

    output
}

/// Merge WebIDL attributes into the regular attributes list.
fn merge_attributes(
    elements: &HashMap<String, ParsedElement>,
    interfaces: &HashMap<String, ParsedInterface>,
) -> HashMap<String, Vec<Attribute>> {
    let mut output = elements
        .iter()
        .map(|(name, _)| (name.clone(), vec![]))
        .collect::<HashMap<_, _>>();

    let interface_map = interfaces
        .iter()
        .map(|(name, interface)| {
            let map = interface
                .attributes
                .iter()
                .map(|attr| (attr.name.to_lowercase().clone(), attr.clone()))
                .collect::<HashMap<String, Attribute>>();
            (name.clone(), map)
        })
        .collect::<HashMap<String, _>>();

    for (_, el) in elements {
        let interface = match interface_map.get(&el.dom_interface) {
            Some(interface) => interface,
            None => {
                let vec = output.entry(el.tag_name.clone()).or_default();
                vec.extend(el.attributes.iter().cloned());
                continue;
            }
        };

        for attr in &el.attributes {
            let attr = match interface.get(&attr.name) {
                Some(other) => Attribute {
                    name: attr.name.clone(),
                    description: attr.description.clone(),
                    field_name: other.field_name.clone(),
                    ty: other.ty.clone(),
                },
                None => attr.clone(),
            };
            let vec = output.entry(el.tag_name.clone()).or_default();
            vec.push(attr);
        }
    }
    output
}
