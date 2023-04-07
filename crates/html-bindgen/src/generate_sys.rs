use std::fmt::Write;
use std::{collections::HashMap, iter};

use crate::ParsedNode;

use super::{parse::Attribute, types};
use indoc::{formatdoc, writedoc};

/// A generated code file, returned so it can be written to disk.
#[derive(Debug)]
pub struct CodeFile {
    pub filename: String,
    pub dir: String,
    pub code: String,
}

const TRAIT: &str = "
/// Render an element to a writer.
pub trait RenderElement {
    /// Write the opening tag to a writer.
    fn write_opening_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;

    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;
}";

// const SHARED_ATTRIBUTES: &str = r#"
// /// The "global attributes" struct
// pub struct GlobalAttributes {

// }
// "#;

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
    dirs.sort();

    // generate `lib.rs` file
    output.push(CodeFile {
        filename: "lib.rs".to_owned(),
        code: dirs
            .into_iter()
            .map(|d| format!("pub mod {d};\n"))
            .chain(iter::once(TRAIT.to_owned()))
            .collect(),
        dir: String::new(),
    });

    Ok(output)
}

/// Generate a single element.
fn generate_element(el: ParsedNode) -> CodeFile {
    let dir = el.element_kind.clone();
    let ParsedNode {
        tag_name,
        struct_name,
        has_closing_tag,
        attributes,
        mdn_link,
        ..
    } = el;

    let filename = format!("{}.rs", tag_name);
    let fields = generate_fields(&attributes);
    let opening_tag_content = generate_opening_tag(&attributes, &tag_name);
    let closing_tag_content = generate_closing_tag(&tag_name, has_closing_tag);

    let code = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        pub struct {struct_name} {{
            {fields}
        }}

        impl crate::RenderElement for {struct_name} {{
            fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {{
                {opening_tag_content}
                Ok(())
            }}

            #[allow(unused_variables)]
            fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {{
                {closing_tag_content}
                Ok(())
            }}
        }}
    "#
    );

    CodeFile {
        filename,
        code,
        dir,
    }
}

fn generate_fields(attributes: &[Attribute]) -> String {
    let mut output = String::new();
    for attr in attributes {
        let description = &attr.description;
        let field_name = &attr.field_name;
        output.push_str(&formatdoc!(
            "/// {description}
             pub {field_name}: std::option::Option<String>,
            "
        ));
    }
    output
}

fn generate_opening_tag(attributes: &[Attribute], tag_name: &str) -> String {
    let mut output = formatdoc!(
        r#"
        write!(writer, "<{tag_name}")?;
    "#
    );
    for attr in attributes {
        let field_name = &attr.field_name;
        let name = &attr.name;
        output.push_str(&formatdoc!(
            r##"if let Some(field) = self.{field_name}.as_ref() {{
                    write!(writer, r#""{name}="{{}}""#, field)?;
                }}
            "##
        ));
    }
    writedoc!(&mut output, r#"write!(writer, ">")?;"#).unwrap();
    output
}

fn generate_closing_tag(tag_name: &str, has_closing_tag: bool) -> String {
    if has_closing_tag {
        formatdoc!(
            r#"write!(writer, "</{tag_name}>")?;
        "#
        )
    } else {
        String::new()
    }
}
