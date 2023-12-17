pub mod html;
pub mod sys;

use crate::Result;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub description: String,
}

/// Map modules to the MDN hierarchy
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleMapping {
    pub name: String,
    pub children: Vec<String>,
}

/// A generated code file, returned so it can be written to disk.
#[derive(Debug)]
pub struct CodeFile {
    pub filename: String,
    pub dir: String,
    pub code: TokenStream,
}

impl CodeFile {
    pub fn code(&self) -> Result<String> {
        let file = syn::parse_file(&self.code.to_string())?;
        Ok(prettyplease::unparse(&file))
    }
}
