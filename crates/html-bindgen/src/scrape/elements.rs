use crate::Result;
use scraper::ElementRef;
use std::collections::HashMap;

/// The raw values extracted from the HTML spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScrapedElement {
    pub tag_name: String,
    pub categories: Vec<String>,
    pub submodule_name: String,
    pub contexts: Vec<String>,
    pub content_model: Vec<String>,
    pub tag_omission: Vec<String>,
    pub content_attributes: Vec<String>,
    pub dom_interface: Vec<String>,
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
pub fn scrape_elements(spec: String) -> Result<Vec<ScrapedElement>> {
    let document = scraper::Html::parse_document(&spec);
    let selector = scraper::Selector::parse(".element").unwrap();

    let extract_text = |child| {
        let categories = ElementRef::wrap(child).unwrap();
        categories.text().collect::<String>()
    };

    let mut specs = vec![];

    for element in document.select(&selector).into_iter() {
        let tag_names = match extract_tag_names(element) {
            Some(tag_names) => tag_names,
            None => continue,
        };

        let element_kind = extract_element_kind(element);

        // Iterate over the table and extract the raw values
        let mut current: Option<(String, Vec<String>)> = None;
        let mut outputs: HashMap<String, Vec<String>> = HashMap::new();
        for child in element.children() {
            let el = child.value().as_element();
            let tag_name = el.as_deref().unwrap().name();
            match tag_name {
                "dt" => {
                    if current.is_some() {
                        let current = current.take().unwrap();
                        outputs.insert(current.0, current.1);
                    }
                    current = Some((extract_text(child), vec![]));
                }
                "dd" => {
                    let current = current.as_mut().unwrap();
                    current.1.push(extract_text(child));
                }
                other => panic!("unexpected tag name {other}"),
            }
        }
        if current.is_some() {
            let current = current.take().unwrap();
            outputs.insert(current.0, current.1);
        }

        // Construct a raw spec item from the parsed data.
        for tag_name in tag_names {
            let tag_omission = match outputs.get("Tag omission in text/html:").as_deref() {
                Some(vec) => vec.clone(),
                None => vec![],
            };

            specs.push(ScrapedElement {
                tag_name,
                submodule_name: element_kind.clone(),
                categories: outputs.get("Categories:").as_deref().unwrap().clone(),
                contexts: outputs
                    .get("Contexts in which this element can be used:")
                    .as_deref()
                    .unwrap()
                    .clone(),
                content_model: outputs.get("Content model:").as_deref().unwrap().clone(),
                content_attributes: outputs
                    .get("Content attributes:")
                    .as_deref()
                    .unwrap()
                    .clone(),
                tag_omission,
                dom_interface: outputs.get("DOM interface:").as_deref().unwrap().clone(),
            });
        }
    }
    Ok(specs)
}

/// Extract the tag names from the document.
fn extract_tag_names(element: scraper::ElementRef) -> Option<Vec<String>> {
    // Find the name of the element we're inspecting.
    let mut sibling = element.prev_sibling().unwrap();
    loop {
        if let scraper::node::Node::Element(element) = sibling.value() {
            if element.name() == "h4" {
                let s = element.id.as_ref().expect("could not parse h4 element id");

                // Skip over `h4` elements which aren't nodes.
                if s.contains("the") && s.contains("element") {
                    return Some(parse_tag_names(s));
                } else {
                    return None;
                }
            }
        }

        sibling = sibling.prev_sibling().unwrap();
    }
}

/// Extract the tag names from the document.
fn extract_element_kind(element: scraper::ElementRef) -> String {
    // Find the name of the element we're inspecting.
    let mut sibling = element.prev_sibling().unwrap();
    loop {
        if let scraper::node::Node::Element(element) = sibling.value() {
            if element.name() == "h3" {
                let s = element.id.as_ref().expect("could not parse h3 element id");
                return s.to_string();
            }
        }

        sibling = sibling.prev_sibling().unwrap();
    }
}

/// Parse the HTML tag names.
///
/// A single HTML heading can correspond to several HTML nodes
fn parse_tag_names(s: &str) -> Vec<String> {
    if s.ends_with("elements") {
        s.strip_prefix("the-")
            .unwrap()
            .strip_suffix("-elements")
            .unwrap()
            .replace("-and", "")
            .replace(",", "")
            .split("-")
            .map(|s| s.to_owned())
            .collect()
    } else {
        let s = s
            .strip_prefix("the-")
            .unwrap()
            .strip_suffix("-element")
            .unwrap()
            .to_owned();
        vec![s]
    }
}
