use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

/// An HTML Node Definition
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Definition {
    pub tag_name: String,
    pub dom_interface: String,
    pub inherits_from: Option<String>,
    pub members: Vec<Attribute>,
}
impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = toml::to_string(&self).unwrap();
        write!(f, "{}", s)?;
        Ok(())
    }
}

/// An HTML attribute
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub read_only: bool,
    pub ty: AttributeTy,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AttributeTy {
    Bool,
    String,
    Integer,
    Float,
    Identifier(String),
}

impl Display for AttributeTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeTy::Bool => write!(f, "bool"),
            AttributeTy::String => write!(f, "String"),
            AttributeTy::Integer => write!(f, "u64"),
            AttributeTy::Float => write!(f, "f64"),
            AttributeTy::Identifier(s) => write!(f, "{s}"),
        }
    }
}
