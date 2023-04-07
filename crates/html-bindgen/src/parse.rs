use crate::types;
use crate::ScrapedNode;
use convert_case::{Case, Casing};

/// The parsed values converted from the raw spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedNode {
    pub tag_name: String,
    pub struct_name: String,
    pub has_closing_tag: bool,
    pub attributes: Vec<Attribute>,
    pub element_kind: String,
    pub mdn_link: String,
}

/// An attribute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub field_name: String,
}

const GLOBAL_ATTRIBUTES: [&str; 27] = [
    "accesskey",
    "autocapitalize",
    "autofocus",
    "class",
    "contenteditable",
    "dir",
    "draggable",
    "enterkeyhint",
    "hidden",
    "id",
    "inert",
    "inputmode",
    "is",
    "itemid",
    "itemprop",
    "itemref",
    "itemscope",
    "itemtype",
    "lang",
    "nonce",
    "popover",
    "slot",
    "spellcheck",
    "style",
    "tabindex",
    "title",
    "translate",
];

pub fn parse(
    scraped: impl Iterator<Item = types::Result<ScrapedNode>>,
) -> types::Result<Vec<ParsedNode>> {
    let mut output = vec![];
    for scraped in scraped {
        let scraped = scraped?;
        let tag_name = scraped.tag_name;
        let mdn_link = parse_mdn_link(&tag_name);
        let struct_name = parse_struct_name(&tag_name);
        let has_closing_tag = parse_tags(scraped.tag_omission);
        let attributes = parse_attrs(scraped.content_attributes);
        let element_kind = parse_kinds(scraped.element_kind);
        output.push(ParsedNode {
            tag_name,
            struct_name,
            has_closing_tag,
            attributes,
            element_kind,
            mdn_link,
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
        "q" => "InlineQuotation".to_owned(),
        "rp" => "RubyFallbackParenthesis".to_owned(),
        "rt" => "RubyText".to_owned(),
        "rtc" => "RubyTextContainer".to_owned(),
        "ruby" => "RubyAnnotation".to_owned(),
        "s" => "StrikeThrough".to_owned(),
        "samp" => "SampleOutput".to_owned(),
        "small" => "SideComment".to_owned(),
        "source" => "MediaSource".to_owned(),
        "span" => "ContentSpan".to_owned(),
        "sub" => "SubScript".to_owned(),
        "sup" => "SuperScript".to_owned(),
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

fn parse_attrs(content_attributes: Vec<String>) -> Vec<Attribute> {
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
        let field_name = normalize_field_name(&name);

        output.push(Attribute {
            name,
            description,
            field_name,
        });
    }
    if has_global_attributes {
        for attr in GLOBAL_ATTRIBUTES {
            output.push(Attribute {
                field_name: normalize_field_name(&attr),
                name: attr.to_owned(),
                description: String::new(),
            })
        }
    }
    output
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

fn normalize_field_name(name: &str) -> String {
    match name.to_case(Case::Snake).as_str() {
        "class" => "class_".to_owned(),
        "loop" => "loop_".to_owned(),
        "type" => "type_".to_owned(),
        "for" => "for_".to_owned(),
        "as" => "as_".to_owned(),
        "is" => "is_".to_owned(),
        "async" => "async_".to_owned(),
        "contenteditable" => "content_editable".to_owned(),
        "accesskey" => "access_key".to_owned(),
        "autocapitalize" => "auto_capitalize".to_owned(),
        "enterkeyhint" => "enter_key_hint".to_owned(),
        "inputmode" => "input_mode".to_owned(),
        "itemid" => "item_id".to_owned(),
        "itemprop" => "item_prop".to_owned(),
        "itemref" => "item_ref".to_owned(),
        "itemscope" => "item_scope".to_owned(),
        "itemtype" => "item_type".to_owned(),
        "tabindex" => "tab_index".to_owned(),
        "dir" => "direction".to_owned(),
        other => other.to_owned(),
    }
}
