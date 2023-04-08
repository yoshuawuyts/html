use std::fmt::Write;
use std::{collections::HashMap, iter};

use super::{CodeFile, Module};
use crate::parse::{Attribute, ParsedElement};
use crate::{utils, Result};
use indoc::{formatdoc, writedoc};

pub fn generate(
    parsed: impl Iterator<Item = Result<ParsedElement>>,
    global_attributes: &[Attribute],
    modules: &[Module],
) -> Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut generated: HashMap<String, Vec<String>> = HashMap::new();

    // generate individual `{element}.rs` files
    for el in parsed {
        let el = el?;
        let entry = generated.entry(el.element_kind.clone());
        entry.or_default().push(el.tag_name.clone());
        let cf = generate_element(el)?;
        output.push(cf);
    }

    // generate `mod.rs` files
    let mut dirs = vec![];
    for (dir, filenames) in generated {
        dirs.push(dir.clone());
        let code = filenames
            .into_iter()
            .map(|name| format!("mod {name};\npub use {name}::*;"))
            .collect::<String>();

        let module = modules.iter().find(|el| &el.name == &dir).unwrap();
        let description = &module.description;
        let code = format!(
            "//! {description}
            {code}"
        );

        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code: utils::fmt(dbg!(&code)).expect("could not parse code"),
            dir,
        })
    }
    dirs.sort();

    // generate `elements/mod.rs` file
    let code = dirs
        .into_iter()
        .map(|d| format!("pub mod {d};\n"))
        .collect::<String>();
    let code = format!(
        "//! HTML elements support
        {code}"
    );
    output.push(CodeFile {
        filename: "mod.rs".to_owned(),
        code: utils::fmt(&code)?,
        dir: String::new(),
    });

    Ok(output)
}

/// Generate a single element.
fn generate_element(el: ParsedElement) -> Result<CodeFile> {
    let dir = el.element_kind.clone();
    let ParsedElement {
        tag_name,
        struct_name,
        has_closing_tag,
        attributes,
        mdn_link,
        has_global_attributes,
        element_kind,
        categories,
        content_model,
        contexts,
    } = el;

    let filename = format!("{}.rs", tag_name);

    let mut code = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        pub struct {struct_name} {{
            sys: html_sys::{element_kind}::{struct_name},
        }}
    "#
    );

    Ok(CodeFile {
        filename,
        code: utils::fmt(&code)?,
        dir,
    })
}
