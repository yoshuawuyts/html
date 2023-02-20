use std::fmt::Display;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An HTML Node Definition
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Definition {
    pub(crate) name: String,
    pub(crate) inherits_from: Option<String>,
    pub(crate) members: Vec<Attribute>,
}

/// An HTML attribute
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Attribute {
    pub(crate) name: String,
    pub(crate) read_only: bool,
    pub(crate) ty: AttributeTy,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AttributeTy {
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
