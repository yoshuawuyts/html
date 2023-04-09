#![allow(unused)]

use std::fmt::Write;
use std::{collections::HashMap, iter};

use super::{CodeFile, Module};
use crate::parse::{Attribute, Category, ParsedElement};
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
        let entry = generated.entry(el.submodule_name.clone());
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
            .map(|name| format!("mod {name};\npub use self::{name}::*;"))
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
    let dir = el.submodule_name.clone();
    let ParsedElement {
        tag_name,
        struct_name,
        has_closing_tag,
        attributes,
        mdn_link,
        has_global_attributes,
        submodule_name,
        content_categories,
        permitted_content,
        permitted_parents,
        dom_interface,
    } = el;

    let filename = format!("{}.rs", tag_name);

    let (bound, generic) = match permitted_content.len() {
        0 => (String::new(), String::new()),
        1 => {
            let s = format_content_model(&permitted_content[0]).to_owned();
            (format!("<T: {s}>",), "<T>".to_owned())
        }
        other => panic!("panicked: {other}"),
    };
    let categories = generate_categories(&content_categories, &struct_name, &bound, &generic);

    let children = match permitted_content.len() {
        0 => String::new(),
        _ => "_children: Vec<T>".to_owned(),
    };

    let gen_children = match permitted_content.len() {
        0 => String::new(),
        _ => "_children: vec![]".to_owned(),
    };

    let sys_name = format!("html_sys::{submodule_name}::{struct_name}");

    let mut code = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        pub struct {struct_name} {bound} {{
            sys: {sys_name},
            {children}
        }}

        {categories}

        impl{bound} std::convert::Into<{sys_name}> for {struct_name} {generic} {{
            fn into(self) -> {sys_name} {{
                self.sys
            }}
        }}

        impl{bound} From<{sys_name}> for {struct_name} {generic} {{
            fn from(sys: {sys_name}) -> Self {{
                Self {{
                    sys,
                    {gen_children}
                }}
            }}
        }}
    "#
    );

    Ok(CodeFile {
        filename,
        code: utils::fmt(&code)?,
        dir,
    })
}

fn generate_categories(
    categories: &[Category],
    struct_name: &str,
    bound: &str,
    generic: &str,
) -> String {
    let mut output = String::new();
    for cat in categories {
        generate_category(cat, &mut output, struct_name, bound, generic);
    }
    output
}

fn generate_category(
    cat: &Category,
    output: &mut String,
    struct_name: &str,
    bound: &str,
    generic: &str,
) {
    match cat {
        Category::Metadata => output.push_str(&format!(
            "impl{bound} crate::categories::MetadataContent for {struct_name}{generic} {{}}"
        )),
        Category::Flow => output.push_str(&format!(
            "impl{bound} crate::categories::FlowContent for {struct_name}{generic} {{}}"
        )),
        Category::Sectioning => {
            output.push_str(&format!(
                "impl{bound} crate::categories::SectioningContent for {struct_name}{generic} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Heading => {
            output.push_str(&format!(
                "impl{bound} crate::categories::HeadingContent for {struct_name}{generic} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Phrasing => {
            output.push_str(&format!(
                "impl{bound} crate::categories::PhrasingContent for {struct_name}{generic} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Embedded => {
            output.push_str(&format!(
                "impl{bound} crate::categories::EmbeddedContent for {struct_name}{generic} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Interactive => {
            output.push_str(&format!(
                "impl{bound} crate::categories::InteractiveContent for {struct_name}{generic} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Palpable => output.push_str(&format!(
            "impl{bound} crate::categories::PalpableContent for {struct_name}{generic} {{}}"
        )),
        Category::ScriptSupporting => output.push_str(&format!(
            "impl{bound} crate::categories::ScriptSupportingContent for {struct_name}{generic} {{}}"
        )),
    }
}

fn format_content_model(cat: &Category) -> &'static str {
    match cat {
        Category::Metadata => "crate::categories::MetadataContent",
        Category::Flow => "crate::categories::FlowContent",
        Category::Flow => "crate::categories::FlowContent",
        Category::Sectioning => "crate::categories::SectioningContent",
        Category::Heading => "crate::categories::HeadingContent",
        Category::Phrasing => "crate::categories::PhrasingContent",
        Category::Embedded => "crate::categories::EmbeddedContent",
        Category::Interactive => "crate::categories::InteractiveContent",
        Category::Palpable => "crate::categories::PalpableContent",
        Category::ScriptSupporting => "crate::categories::ScriptSupportingContent",
    }
}
