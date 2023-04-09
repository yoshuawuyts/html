mod elements;
mod webidls;

pub use elements::{parse_elements, ParsedElement};
pub use webidls::{parse_webidls, ParsedInterface};

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            AttributeType::Identifier(_) => todo!("identifier attr not yet implemented"),
            AttributeType::Enumerable(_) => todo!("enum attr not yet implemented"),
        }
    }
}

/// Each element in HTML falls into zero or more categories that group elements with similar characteristics together
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Category {
    Metadata,
    Flow,
    Sectioning,
    Heading,
    Phrasing,
    Embedded,
    Interactive,
    Palpable,
    ScriptSupporting,
}
