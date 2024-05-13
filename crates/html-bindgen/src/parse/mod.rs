mod aria;
mod elements;
mod webidls;

pub use aria::{
    parse_aria_elements, parse_aria_properties, parse_aria_roles, ParsedAriaElement,
    ParsedAriaProperty, ParsedAriaRole,
};
use convert_case::{Case, Casing};
pub use elements::{parse_elements, parse_struct_name, ParsedElement};
pub use webidls::{parse_webidls, ParsedInterface};

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An attribute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub field_name: String,
    pub ty: AttributeType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    Bool,
    String,
    Integer,
    Float,
    Identifier(String),
    Enumerable(Vec<String>),
}

impl Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeType::Bool => write!(f, "bool"),
            AttributeType::String => write!(f, "String"),
            AttributeType::Integer => write!(f, "i64"),
            AttributeType::Float => write!(f, "f64"),
            AttributeType::Identifier(i) => write!(f, "{}", i),
            AttributeType::Enumerable(_) => todo!("enum attr not yet implemented"),
        }
    }
}

/// Each element in HTML falls into zero or more categories that group elements
/// with similar characteristics together.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ParsedCategory {
    Metadata,
    Flow,
    Sectioning,
    Heading,
    Phrasing,
    Embedded,
    Interactive,
    Palpable,
    ScriptSupporting,
    Transparent,
}

/// Each element in HTML has zero or more relationships to other elements.
///
/// This also holds the custom "Element" relationship, which is used to
/// represent specific elements.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ParsedRelationship {
    Element(String),
    Category(ParsedCategory),
}

impl From<ParsedCategory> for ParsedRelationship {
    fn from(value: ParsedCategory) -> Self {
        ParsedRelationship::Category(value)
    }
}

pub fn normalize_field_name(name: &str) -> String {
    match name.to_case(Case::Snake).as_str() {
        "loop" => "loop_".to_owned(),
        "type" => "type_".to_owned(),
        "for" => "for_".to_owned(),
        "as" => "as_".to_owned(),
        "async" => "async_".to_owned(),
        other => other.to_owned(),
    }
}
