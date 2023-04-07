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
    pub dir: String,
    pub code: String,
}

pub fn generate(
    parsed: impl Iterator<Item = types::Result<ParsedNode>>,
) -> types::Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut generated: HashMap<String, Vec<String>> = HashMap::new();

    // generate individual html files
    for el in parsed {
        let el = el?;
        // generate the various individual item files
        let dir = el.element_kind.clone();
        let filename = format!("{}.rs", el.tag_name);
        generated
            .entry(el.element_kind)
            .or_default()
            .push(el.tag_name);

        let code = String::new();

        output.push(CodeFile {
            filename,
            code,
            dir,
        })
    }

    // generate mod.rs files
    let mut dirs = vec![];
    for (dir, filenames) in generated {
        dirs.push(dir.clone());
        let code = filenames
            .into_iter()
            .map(|name| format!("mod {name};\npub use {name}::*;"))
            .collect::<Vec<String>>()
            .join("\n");
        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code,
            dir,
        })
    }

    // generate lib.rs
    let code = dirs
        .into_iter()
        .map(|d| format!("pub mod {d};\n"))
        .collect();
    output.push(CodeFile {
        filename: "lib.rs".to_owned(),
        code,
        dir: String::new(),
    });

    Ok(output)
}
