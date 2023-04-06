use std::{collections::HashSet, fs, path::Path};

mod generate;
mod parse;
mod types;

use generate::def_to_string;
use scraper::{ElementRef, Node, Selector};
use types::*;

pub type Database = HashSet<Definition>;

pub fn generate_html(path: &Path) -> types::Result<String> {
    let database = parse(path)?;

    let mut output = String::new();
    output.push_str(
        "/// An HTML Element
        pub trait HtmlElement: ::std::fmt::Display {}\n
        ",
    );
    for entry in database {
        output.push_str(&def_to_string(entry));
    }

    // missing types in the IDL files!
    output.push_str(
        "#[derive(Default, Debug, PartialEq, Clone)]
        pub struct FileList {}
        
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct EventTarget {}
        ",
    );
    Ok(output)
}

pub fn parse(path: &Path) -> types::Result<Database> {
    let iter = fs::read_dir(path)?
        .into_iter()
        .map(|path| fs::read_to_string(path?.path()));
    let database = parse::parse_webidl(iter)?;

    Ok(database)
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
pub fn parse_spec(spec: String) -> types::Result<()> {
    let document = scraper::Html::parse_document(&spec);
    let selector = scraper::Selector::parse(".element").unwrap();

    let extract_text = |child| {
        let categories = ElementRef::wrap(child).unwrap();
        categories.text().collect::<String>()
    };

    let mut specs = vec![];

    for element in document.select(&selector) {
        let tag_names = extract_tag_names(element);
        let mut children = element.children();

        children.next().unwrap();
        let categories = extract_text(children.next().unwrap());

        children.next().unwrap();
        let contexts = extract_text(children.next().unwrap());

        children.next().unwrap();
        children.next().unwrap();
        let content_model = extract_text(children.next().unwrap());

        children.next().unwrap();
        let tag_omission = extract_text(children.next().unwrap());

        children.next().unwrap();
        let content_attributes = extract_text(children.next().unwrap());

        children.next().unwrap();
        let dom_interface = extract_text(children.next().unwrap());

        for tag_name in tag_names {
            specs.push(RawSpec {
                tag_name,
                categories: categories.clone(),
                contexts: contexts.clone(),
                content_model: content_model.clone(),
                tag_omission: tag_omission.clone(),
                content_attributes: content_attributes.clone(),
                dom_interface: dom_interface.clone(),
            });
        }

        dbg!(specs);

        // for text in categories.value().text() {
        //     dbg!(text);
        // }
        std::process::exit(1);

        // if let scraper::node::Node::Element(element) = categories.value() {
        //     dbg!(element);
        //     std::process::exit(1);
        //     if element.name() == "dd" {
        //         let s = element.id.as_ref().expect("could not parse dd element");
        //         dbg!(element);
        //     } else {
        //         panic!("wrong node type!")
        //     }
        // }

        // TODO: process the HTML table here.
        // dbg!(element.value());
    }
    Ok(())
}

/// The raw values extracted from the HTML spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RawSpec {
    tag_name: String,
    categories: String,
    contexts: String,
    content_model: String,
    tag_omission: String,
    content_attributes: String,
    dom_interface: String,
}

/// Extract the tag names from the document.
fn extract_tag_names(element: scraper::ElementRef) -> Vec<String> {
    // Find the name of the element we're inspecting.
    let mut sibling = element.prev_sibling().unwrap();
    loop {
        if let scraper::node::Node::Element(element) = sibling.value() {
            if element.name() == "h4" {
                let s = element.id.as_ref().expect("could not parse h4 element id");
                return parse_tag_names(s);
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
        let iter = s
            .strip_prefix("the-")
            .unwrap()
            .strip_suffix("-elements")
            .unwrap()
            .split(",-");
        iter.map(|name| match name.strip_prefix("and-") {
            Some(name) => name.to_owned(),
            None => name.to_owned(),
        })
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
