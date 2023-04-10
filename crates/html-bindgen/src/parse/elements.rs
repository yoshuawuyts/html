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
    scraped: impl Iterator<Item = Result<ScrapedElement>>,
) -> Result<Vec<ParsedElement>> {
    let mut output = vec![];
    for scraped in scraped {
        let scraped = scraped?;
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
            content_categories: parse_categories(&scraped.categories),
            permitted_content: parse_relationships(&scraped.content_model),
            permitted_parents: parse_contexts(&scraped.contexts),
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

fn parse_relationships(categories: &[String]) -> Vec<ParsedRelationship> {
    let mut cat_output = vec![];
    for line in categories {
        if line.starts_with("Phrasing content,") {
            cat_output.push(ParsedRelationship::Phrasing);
            continue;
        }
        match line.as_str() {
            "Metadata content." => cat_output.push(ParsedRelationship::Metadata),
            "Flow content." => cat_output.push(ParsedRelationship::Flow),
            "Sectioning content." => cat_output.push(ParsedRelationship::Sectioning),
            "Heading content." => cat_output.push(ParsedRelationship::Heading),
            "Phrasing content." => cat_output.push(ParsedRelationship::Phrasing),
            "Embedded content." => cat_output.push(ParsedRelationship::Embedded),
            "Interactive content." => cat_output.push(ParsedRelationship::Interactive),
            "Palpable content." => cat_output.push(ParsedRelationship::Palpable),
            other => eprintln!("unknown content kind: {other}"),
        }
    }
    cat_output
}

fn parse_categories(categories: &[String]) -> Vec<ParsedCategory> {
    let mut cat_output = vec![];
    for line in categories {
        if line.starts_with("Phrasing content,") {
            cat_output.push(ParsedCategory::Phrasing);
            continue;
        }
        match line.as_str() {
            "Metadata content." => cat_output.push(ParsedCategory::Metadata),
            "Flow content." => cat_output.push(ParsedCategory::Flow),
            "Sectioning content." => cat_output.push(ParsedCategory::Sectioning),
            "Heading content." => cat_output.push(ParsedCategory::Heading),
            "Phrasing content." => cat_output.push(ParsedCategory::Phrasing),
            "Embedded content." => cat_output.push(ParsedCategory::Embedded),
            "Interactive content." => cat_output.push(ParsedCategory::Interactive),
            "Palpable content." => cat_output.push(ParsedCategory::Palpable),
            other => eprintln!("unknown content kind: {other}"),
        }
    }
    cat_output
}

fn parse_contexts(categories: &[String]) -> Vec<ParsedRelationship> {
    let mut cat_output = vec![];
    for line in categories {
        if !line.starts_with("Where ") {
            continue;
        }
        let s = line.strip_prefix("Where ").unwrap();
        let s = if s.ends_with("is expected.") {
            s.strip_suffix(" is expected.").unwrap()
        } else if s.ends_with("are expected.") {
            s.strip_suffix(" are expected.").unwrap()
        } else {
            eprintln!("unknown content kind: {s}");
            continue;
        };
        match s {
            "metadata content" => cat_output.push(ParsedRelationship::Metadata),
            "flow content" => cat_output.push(ParsedRelationship::Flow),
            "sectioning content" => cat_output.push(ParsedRelationship::Sectioning),
            "heading content" => cat_output.push(ParsedRelationship::Heading),
            "phrasing content" => cat_output.push(ParsedRelationship::Phrasing),
            "embedded content" => cat_output.push(ParsedRelationship::Embedded),
            "interactive content" => cat_output.push(ParsedRelationship::Interactive),
            "palpable content" => cat_output.push(ParsedRelationship::Palpable),
            "script-supporting elements" => cat_output.push(ParsedRelationship::ScriptSupporting),
            other => eprintln!("unknown content kind: {other}"),
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
