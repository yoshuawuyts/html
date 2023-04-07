use std::collections::HashMap;

use crate::ParsedNode;

use super::types;
use indoc::formatdoc;

/// A generated code file, returned so it can be written to disk.
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

    // generate individual `{element}.rs` files
    for el in parsed {
        let el = el?;
        let entry = generated.entry(el.element_kind.clone());
        entry.or_default().push(el.tag_name.clone());
        let cf = generate_element(el);
        output.push(cf);
    }

    // generate `mod.rs` files
    let mut dirs = vec![];
    for (dir, filenames) in generated {
        dirs.push(dir.clone());
        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code: filenames
                .into_iter()
                .map(|name| format!("mod {name};\npub use {name}::*;\n"))
                .collect(),
            dir,
        })
    }

    // generate `lib.rs` file
    output.push(CodeFile {
        filename: "lib.rs".to_owned(),
        code: dirs
            .into_iter()
            .map(|d| format!("pub mod {d};\n"))
            .collect(),
        dir: String::new(),
    });

    Ok(output)
}

fn generate_element(el: ParsedNode) -> CodeFile {
    let dir = el.element_kind.clone();
    let ParsedNode {
        tag_name,
        struct_name,
        has_opening_tag,
        has_closing_tag,
        has_global_attributes,
        attributes,
        element_kind,
        mdn_link,
    } = el;
    let filename = format!("{}.rs", tag_name);

    let fields = attributes
        .into_iter()
        .map(|attr| format!("{}: String,\n", attr.field_name))
        .collect::<String>();

    let code = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        pub struct {struct_name} {{
            {fields}
        }}
    "#
    );

    CodeFile {
        filename,
        code,
        dir,
    }
}
