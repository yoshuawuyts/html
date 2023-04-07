#![allow(dead_code)]

use std::collections::HashMap;

use crate::ParsedNode;

use super::types;
// use convert_case::{Case, Casing};
// use indoc::formatdoc;

/// A generated code file
#[derive(Debug)]
pub struct CodeFile {
    pub filename: String,
    pub code: String,
}

pub fn generate(
    parsed: impl Iterator<Item = types::Result<ParsedNode>>,
) -> types::Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut generated: HashMap<String, Vec<String>> = HashMap::new();

    for el in parsed {
        let el = el?;
        // generate the various individual item files
        let filename = format!("{}/{}.rs", el.element_kind, el.tag_name);
        generated
            .entry(el.element_kind)
            .or_default()
            .push(el.tag_name);

        let code = String::new();

        output.push(CodeFile { filename, code })
    }

    for (dir, filenames) in generated {
        let code = filenames
            .into_iter()
            .map(|name| format!("mod {name};\npub use {name}::*;"))
            .collect::<Vec<String>>()
            .join("\n");
        let filename = format!("{}/mod.rs", dir);
        output.push(CodeFile { filename, code })
    }

    Ok(output)
}
