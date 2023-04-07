use crate::types;
use crate::ScrapedNode;

/// The parsed values converted from the raw spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedNode {
    pub tag_name: String,
    pub has_opening_tag: bool,
    pub has_closing_tag: bool,
    pub has_global_attributes: bool,
    pub attributes: Vec<Attribute>,
}

/// An attribute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    pub name: String,
    pub description: String,
}

pub fn parse(
    scraped: impl Iterator<Item = types::Result<ScrapedNode>>,
) -> types::Result<Vec<ParsedNode>> {
    let mut output = vec![];
    for scraped in scraped {
        let scraped = scraped?;
        let tag_name = dbg!(scraped.tag_name);
        let (has_opening_tag, has_closing_tag) = parse_tags(scraped.tag_omission);
        let (has_global_attributes, attributes) = parse_attrs(scraped.content_attributes);
        output.push(ParsedNode {
            tag_name,
            has_opening_tag,
            has_closing_tag,
            has_global_attributes,
            attributes,
        });
    }
    Ok(output)
}

fn parse_tags(input: Vec<String>) -> (bool, bool) {
    let s = input.join("");
    match &*s {
        "Neither tag is omissible." | "" => (true, true),
        "No end tag." => (true, false),
        _ => (true, true),
    }
}

fn parse_attrs(content_attributes: Vec<String>) -> (bool, Vec<Attribute>) {
    let mut has_global_attributes = false;
    let mut output = vec![];
    for s in content_attributes {
        dbg!(&s);
        if s == "Global attributes" {
            has_global_attributes = true;
            continue;
        } else if !s.contains("—") {
            continue;
        }
        let mut iter = s.split("—");
        let name = iter.next().unwrap().trim().to_owned();
        let description = iter.next().unwrap().trim().to_owned();
        output.push(Attribute { name, description });
    }
    (has_global_attributes, output)
}
