use crate::scrape::ScrapedElement;
use crate::Result;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use super::{Attribute, AttributeType, ParsedCategory, ParsedRelationship};

/// The parsed values converted from the raw spec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedElement {
    pub tag_name: String,
    pub struct_name: String,
    pub submodule_name: String,
    pub mdn_link: String,
    pub has_global_attributes: bool,
    pub has_closing_tag: bool,
    pub attributes: Vec<Attribute>,
    pub dom_interface: String,
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
        output.push(ParsedElement {
            struct_name,
            dom_interface,
            has_closing_tag: parse_tags(scraped.tag_omission),
            attributes,
            has_global_attributes,
            submodule_name: parse_kinds(scraped.submodule_name),
            mdn_link: parse_mdn_link(&tag_name),
            content_categories: parse_content_categories(&scraped.categories),
            permitted_content: parse_relationships(&scraped.content_model, &tag_names),
            permitted_parents: parse_relationships(&scraped.contexts, &tag_names),
            tag_name,
        });
    }
    Ok(output)
}

fn parse_mdn_link(tag_name: &str) -> String {
    format!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/{tag_name}")
}

fn parse_struct_name(tag_name: &str) -> String {
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

        // we skip over all the conditional comments for now.
        if name.contains(' ') {
            continue;
        }

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

fn parse_categories(mut line: &str) -> Vec<String> {
    if line.contains("Zero or more") {
        dbg!(line);
        let mut iter = line.split("Zero or more");
        let _ = iter.next().unwrap();
        line = iter.next().unwrap();
        let mut output = vec![];
        for word in line.split("and") {
            output.push(parse_category(word));
        }
        output
    } else {
        vec![parse_category(line)]
    }
}

fn parse_category(mut line: &str) -> String {
    if line.starts_with("If the") {
        if line.contains(":") {
            let mut iter = line.split(":");
            _ = iter.next().unwrap();
            line = iter.next().unwrap();
        }
    }

    if line.starts_with("Where ") {
        line = line.strip_prefix("Where ").unwrap();
        if line.ends_with("is expected.") {
            line = line.strip_suffix(" is expected.").unwrap();
        } else if line.ends_with("are expected.") {
            line = line.strip_suffix(" are expected.").unwrap();
        };
    }

    if line.contains(",") {
        let mut iter = line.split(",");
        line = iter.next().unwrap();
    }
    if line.contains(".") {
        let mut iter = line.split(".");
        line = iter.next().unwrap();
    }

    if line.contains("content") {
        let mut iter = line.split("content");
        line = iter.next().unwrap();
    }

    if line.contains("elements") {
        let mut iter = line.split("elements");
        line = iter.next().unwrap();
    }

    line.trim().to_lowercase().to_owned()
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
    cat_output
}

fn parse_relationships(categories: &[String], tag_names: &[String]) -> Vec<ParsedRelationship> {
    let mut cat_output = vec![];
    for line in categories {
        for line in parse_categories(dbg!(line.as_str())) {
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
                    if tag_names.contains(dbg!(&tag_name.to_owned())) {
                        cat_output.push(ParsedRelationship::Element(parse_struct_name(tag_name)));
                    } else {
                        eprintln!("unknown content kind: {tag_name}");
                    }
                }
            }
        }
    }
    cat_output
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
