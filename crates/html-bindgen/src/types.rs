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
    pub members: Vec<Member>,
}

/// An HTML attribute
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub read_only: bool,
    pub ty: MemberType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MemberType {
    Bool,
    String,
    Integer,
    Float,
    Identifier(String),
}
