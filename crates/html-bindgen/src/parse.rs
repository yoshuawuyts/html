use crate::types;
use crate::ScrapedNode;
use convert_case::{Case, Casing};

/// The parsed values converted from the raw spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedNode {
    pub tag_name: String,
    pub struct_name: String,
    pub has_opening_tag: bool,
    pub has_closing_tag: bool,
    pub has_global_attributes: bool,
    pub attributes: Vec<Attribute>,
    pub element_kind: String,
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
        let tag_name = scraped.tag_name;
        let struct_name = parse_struct_name(&tag_name);
        let (has_opening_tag, has_closing_tag) = parse_tags(scraped.tag_omission);
        let (has_global_attributes, attributes) = parse_attrs(scraped.content_attributes);
        let element_kind = parse_kinds(scraped.element_kind);
        output.push(ParsedNode {
            tag_name,
            struct_name,
            has_opening_tag,
            has_closing_tag,
            has_global_attributes,
            attributes,
            element_kind,
        });
    }
    Ok(output)
}

fn parse_struct_name(tag_name: &str) -> String {
    match tag_name {
        "a" => "Anchor".to_owned(),
        "abbr" => "Abbreviation".to_owned(),
        "b" => "BringAttention".to_owned(),
        "bdi" => "BidirectionalIsolate".to_owned(),
        "bdo" => "BidirectionalTextOverride".to_owned(),
        "bgsound" => "BackgroundSound".to_owned(),
        "br" => "LineBreak".to_owned(),
        "col" => "TableColumn".to_owned(),
        "colgroup" => "TableColumnGroup".to_owned(),
        "dd" => "DescriptionDetails".to_owned(),
        "del" => "DeletedText".to_owned(),
        "dfn" => "Definition".to_owned(),
        "dir" => "Directory".to_owned(),
        "div" => "ContentDivision".to_owned(),
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
        "i" => "IdiomaticText".to_owned(),
        "ins" => "InsertedText".to_owned(),
        "kdb" => "KeyboardInput".to_owned(),
        "li" => "ListItem".to_owned(),
        "nav" => "Navigation".to_owned(),
        "nobr" => "NonBreakingText".to_owned(),
        "ol" => "OrderedList".to_owned(),
        "optgroup" => "OptionGroup".to_owned(),
        "p" => "Paragraph".to_owned(),
        "pre" => "PreformattedText".to_owned(),
        "q" => "InlineQuotation".to_owned(),
        "rp" => "RubyFallbackParenthesis".to_owned(),
        "rt" => "RubyText".to_owned(),
        "rtc" => "RubyTextContainer".to_owned(),
        "ruby" => "RubyAnnotation".to_owned(),
        "s" => "StrikeThrough".to_owned(),
        "samp" => "SampleOutput".to_owned(),
        "section" => "GenericSection".to_owned(),
        "small" => "SideComment".to_owned(),
        "source" => "MediaSource".to_owned(),
        "span" => "ContentSpan".to_owned(),
        "strong" => "StrongImportance".to_owned(),
        "sub" => "SubScript".to_owned(),
        "summary" => "DisclosureSummary".to_owned(),
        "sup" => "SuperScript".to_owned(),
        "tbody" => "TableBody".to_owned(),
        "td" => "TableCell".to_owned(),
        "tfoot" => "TableFoot".to_owned(),
        "th" => "TableHeader".to_owned(),
        "thead" => "TableHead".to_owned(),
        "tr" => "TableRow".to_owned(),
        "tt" => "TeletypeText".to_owned(),
        "u" => "UnarticulatedAnnotation".to_owned(),
        "ul" => "UnorderedList".to_owned(),
        "var" => "Variable".to_owned(),
        "wbr" => "LineBreakOpportunity".to_owned(),
        other => other.to_case(Case::UpperCamel),
    }
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
