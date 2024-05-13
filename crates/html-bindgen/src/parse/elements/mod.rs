use crate::scrape::ScrapedElement;
use crate::Result;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use super::{Attribute, AttributeType, ParsedCategory, ParsedRelationship};
use categories::parse_categories;

mod categories;

/// The parsed values converted from the raw spec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedElement {
    pub dom_name: String,
    pub dom_base: Option<String>,
    pub tag_name: String,
    pub struct_name: String,
    pub submodule_name: String,
    pub mdn_link: String,
    pub has_global_attributes: bool,
    pub has_closing_tag: bool,
    pub attributes: Vec<Attribute>,
    pub dom_interface: String,
    pub includes: Vec<String>,
    pub content_categories: Vec<ParsedCategory>,
    pub permitted_content: Vec<ParsedRelationship>,
    pub permitted_parents: Vec<ParsedRelationship>,
}

pub fn parse_elements(
    scraped_iter: impl Iterator<Item = Result<ScrapedElement>>,
) -> Result<Vec<ParsedElement>> {
    let mut scraped = vec![];
    for el in scraped_iter {
        let el = el?;
        scraped.push(el);
    }

    let mut tag_names = vec![];
    for scraped in scraped.iter().cloned() {
        // let struct_name = parse_struct_name(&scraped.tag_name);
        tag_names.push(scraped.tag_name.to_owned());
    }
    let mut output = vec![];
    for scraped in scraped {
        let tag_name = scraped.tag_name;
        let struct_name = parse_struct_name(&tag_name);
        let (has_global_attributes, attributes) = parse_attrs(scraped.content_attributes);
        let dom_interface = parse_dom_interface(&scraped.dom_interface);
        let (dom_name, dom_base, includes) = parse_webidl_includes(&scraped.dom_interface);
        let mut permitted_parents = parse_relationships(&scraped.contexts, &tag_names);
        append_super_categories(&mut permitted_parents);
        output.push(ParsedElement {
            dom_name,
            dom_base,
            struct_name,
            dom_interface,
            includes,
            has_closing_tag: parse_tags(scraped.tag_omission),
            attributes,
            has_global_attributes,
            submodule_name: parse_kinds(scraped.submodule_name),
            mdn_link: parse_mdn_link(&tag_name),
            content_categories: parse_content_categories(&scraped.categories),
            permitted_content: parse_relationships(&scraped.content_model, &tag_names),
            permitted_parents,
            tag_name,
        });
    }
    Ok(output)
}

fn parse_mdn_link(tag_name: &str) -> String {
    format!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/{tag_name}")
}

pub fn parse_struct_name(tag_name: &str) -> String {
    match tag_name {
        "a" => "Anchor".to_owned(),
        "abbr" => "Abbreviation".to_owned(),
        "area" => "ImageMapArea".to_owned(),
        "b" => "Bold".to_owned(),
        "bdi" => "BidirectionalIsolate".to_owned(),
        "bdo" => "BidirectionalTextOverride".to_owned(),
        "bgsound" => "BackgroundSound".to_owned(),
        "blockquote" => "BlockQuote".to_owned(),
        "br" => "LineBreak".to_owned(),
        "col" => "TableColumn".to_owned(),
        "colgroup" => "TableColumnGroup".to_owned(),
        "datalist" => "DataList".to_owned(),
        "dd" => "DescriptionDetails".to_owned(),
        "del" => "DeletedText".to_owned(),
        "dfn" => "Definition".to_owned(),
        "dir" => "Directory".to_owned(),
        "div" => "Division".to_owned(),
        "dl" => "DescriptionList".to_owned(),
        "dt" => "DescriptionTerm".to_owned(),
        "em" => "Emphasis".to_owned(),
        "figcaption" => "FigureCaption".to_owned(),
        "hgroup" => "HeadingGroup".to_owned(),
        "h1" => "Heading1".to_owned(),
        "h2" => "Heading2".to_owned(),
        "h3" => "Heading3".to_owned(),
        "h4" => "Heading4".to_owned(),
        "h5" => "Heading5".to_owned(),
        "h6" => "Heading6".to_owned(),
        "hr" => "ThematicBreak".to_owned(),
        "i" => "Italic".to_owned(),
        "img" => "Image".to_owned(),
        "ins" => "InsertedText".to_owned(),
        "kbd" => "KeyboardInput".to_owned(),
        "li" => "ListItem".to_owned(),
        "map" => "ImageMap".to_owned(),
        "mark" => "MarkText".to_owned(),
        "nav" => "Navigation".to_owned(),
        "nobr" => "NonBreakingText".to_owned(),
        "noscript" => "NoScript".to_owned(),
        "ol" => "OrderedList".to_owned(),
        "optgroup" => "OptionGroup".to_owned(),
        "p" => "Paragraph".to_owned(),
        "pre" => "PreformattedText".to_owned(),
        "q" => "Quotation".to_owned(),
        "rp" => "RubyFallbackParenthesis".to_owned(),
        "rt" => "RubyText".to_owned(),
        "rtc" => "RubyTextContainer".to_owned(),
        "ruby" => "RubyAnnotation".to_owned(),
        "s" => "StrikeThrough".to_owned(),
        "samp" => "SampleOutput".to_owned(),
        "small" => "SideComment".to_owned(),
        "source" => "MediaSource".to_owned(),
        "span" => "Span".to_owned(),
        "sub" => "SubScript".to_owned(),
        "sup" => "SuperScript".to_owned(),
        "textarea" => "TextArea".to_owned(),
        "tbody" => "TableBody".to_owned(),
        "td" => "TableCell".to_owned(),
        "tfoot" => "TableFoot".to_owned(),
        "th" => "TableHeader".to_owned(),
        "thead" => "TableHead".to_owned(),
        "tr" => "TableRow".to_owned(),
        "track" => "TextTrack".to_owned(),
        "tt" => "TeletypeText".to_owned(),
        "u" => "Underline".to_owned(),
        "ul" => "UnorderedList".to_owned(),
        "var" => "Variable".to_owned(),
        "wbr" => "LineBreakOpportunity".to_owned(),
        other => other.to_case(Case::UpperCamel),
    }
}

fn parse_tags(input: Vec<String>) -> bool {
    let s = input.join("");
    match &*s {
        "Neither tag is omissible." | "" => true,
        "No end tag." => false,
        // NOTE: There are a bunch of conditional cases which allow omitting end tags
        // but for the sake of convenience we just don't bother with any of those.
        // That's mostly important for parsers, which we're not defining here.
        _ => true,
    }
}

fn parse_attrs(content_attributes: Vec<String>) -> (bool, Vec<Attribute>) {
    let mut has_global_attributes = false;
    let mut output = vec![];
    for s in content_attributes {
        if s == "Global attributes" {
            has_global_attributes = true;
            continue;
        } else if !s.contains("—") {
            continue;
        }
        let mut iter = s.split("—");
        let name = iter.next().unwrap().trim().to_owned();
        let description = iter.next().unwrap().trim().to_owned();

        // Add conditional attributes and document their conditionality.
        // This probably won't be the final way this is done as this loses some of the type-safety guarantees.
        let (name, description) = match name.as_str() {
            "If the element is not a child of an ul or menu element: value" => {
                ("value".to_owned(), format!("{description}. Only if the element is not a child of an `ul` or `menu` element."))
            }
            _ => if let Some((name, condition)) = name.split_once(" ") {
                (name.to_owned(), format!("{description} {condition}"))
            } else {
                (name, description)
            }
        };

        // Rename attributes which are labeled after keywords
        let field_name = super::normalize_field_name(&name);

        output.push(Attribute {
            ty: AttributeType::String,
            name,
            description,
            field_name,
        });
    }
    (has_global_attributes, output)
}

fn parse_kinds(kind: String) -> String {
    let s = match kind.as_str() {
        "the-root-element" => "root",
        "interactive-elements" => "interactive",
        "grouping-content" => "text",
        "text-level-semantics" => "text",
        "document-metadata" => "metadata",
        "embedded-content" => "embedded",
        "forms" => "forms",
        "tables" => "tables",
        "sections" => "sections",
        "edits" => "edits",
        "scripting-3" => "scripting",
        other => panic!("unknown category: {other}"),
    };
    s.to_owned()
}

fn parse_content_categories(categories: &[String]) -> Vec<ParsedCategory> {
    let mut cat_output = vec![];
    for line in categories {
        for line in parse_categories(line.as_str()) {
            match line.as_str() {
                "metadata" => cat_output.push(ParsedCategory::Metadata),
                "flow" => cat_output.push(ParsedCategory::Flow),
                "sectioning" => cat_output.push(ParsedCategory::Sectioning),
                "heading" => cat_output.push(ParsedCategory::Heading),
                "phrasing" => cat_output.push(ParsedCategory::Phrasing),
                "embedded" => cat_output.push(ParsedCategory::Embedded),
                "interactive" => cat_output.push(ParsedCategory::Interactive),
                "palpable" => cat_output.push(ParsedCategory::Palpable),
                "transparent" => cat_output.push(ParsedCategory::Transparent),
                "script-supporting" => cat_output.push(ParsedCategory::ScriptSupporting),
                other => eprintln!("unknown content kind: {other}"),
            }
        }
    }

    cat_output.dedup();
    cat_output.sort();
    cat_output
}

fn parse_relationships(categories: &[String], tag_names: &[String]) -> Vec<ParsedRelationship> {
    let mut cat_output = vec![];
    for line in categories {
        for line in parse_categories(line.as_str()) {
            match line.as_str() {
                "metadata" => cat_output.push(ParsedCategory::Metadata.into()),
                "flow" => cat_output.push(ParsedCategory::Flow.into()),
                "sectioning" => cat_output.push(ParsedCategory::Sectioning.into()),
                "heading" => cat_output.push(ParsedCategory::Heading.into()),
                "phrasing" => cat_output.push(ParsedCategory::Phrasing.into()),
                "embedded" => cat_output.push(ParsedCategory::Embedded.into()),
                "interactive" => cat_output.push(ParsedCategory::Interactive.into()),
                "palpable" => cat_output.push(ParsedCategory::Palpable.into()),
                "transparent" => cat_output.push(ParsedCategory::Transparent.into()),
                "script-supporting" => cat_output.push(ParsedCategory::ScriptSupporting.into()),
                tag_name => {
                    if tag_names.contains(&tag_name.to_owned()) {
                        cat_output.push(ParsedRelationship::Element(parse_struct_name(tag_name)));
                    } else if tag_name == "text" {
                        cat_output.push(ParsedRelationship::Element(parse_struct_name("Text")));
                    } else {
                        eprintln!("unknown tag name: {tag_name}");
                    }
                }
            }
        }
    }

    cat_output.sort();
    cat_output.dedup();
    cat_output
}

fn append_super_categories(cat_output: &mut Vec<ParsedRelationship>) {
    let mut additional_output = vec![];

    for relationship in &*cat_output {
        if let ParsedRelationship::Category(category) = relationship {
            match category {
                ParsedCategory::Heading
                | ParsedCategory::Sectioning
                | ParsedCategory::Phrasing
                | ParsedCategory::Interactive => {
                    additional_output.push(ParsedCategory::Flow.into());
                }
                ParsedCategory::Embedded => {
                    additional_output.push(ParsedCategory::Phrasing.into());
                    additional_output.push(ParsedCategory::Flow.into());
                }
                ParsedCategory::Transparent => {
                    // NOTE(yosh): Sure, why not.
                    additional_output.push(ParsedCategory::Flow.into());
                }
                _ => continue,
            }
        }
    }

    cat_output.append(&mut additional_output);
    cat_output.sort();
    cat_output.dedup();
}

/// Find out which WebIDL interface this element relies on.
fn parse_dom_interface(lines: &[String]) -> String {
    let line = lines.get(0).as_deref().unwrap().clone();

    if line.starts_with("Uses") {
        let line = line.strip_prefix("Uses").unwrap();
        let line = line.strip_suffix(".").unwrap();
        line.trim().to_owned()
    } else if line.starts_with("Use") {
        let line = line.strip_prefix("Use").unwrap();
        let line = line.strip_suffix(".").unwrap();
        line.trim().to_owned()
    } else {
        crate::utils::extract_webidl_name(&line).unwrap()
    }
}

/// Extract what WebIDL interfaces are included
fn parse_webidl_includes(lines: &[String]) -> (String, Option<String>, Vec<String>) {
    let mut name = String::new();
    let mut base = None;

    for line in lines.iter() {
        if let Some(c) = line.split("interface ").skip(1).next() {
            let mut i = c.split(' ');
            if let Some(d) = i.next() {
                name = d.to_string();
                if i.next() == Some(":") {
                    base = i.next().map(|i|i.to_string());
                }
                break;
            }
        }
    }

    let mut includes = Vec::new();
    for line in lines.iter() {
        let check = format!("{} includes ", name);
        for (i, _m) in line.match_indices(&check) {
            let is = line.split_at(i + check.len()).1.to_string();
            let semicolon = is.find(';').unwrap();
            let js = is.split_at(semicolon).0;
            includes.push(js.to_string());
        }
    }
    (name, base, includes)
}
