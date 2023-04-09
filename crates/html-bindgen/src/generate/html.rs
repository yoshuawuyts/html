#![allow(unused)]

use std::fmt::Write;
use std::{collections::HashMap, iter};

use super::{CodeFile, Module};
use crate::merge::MergedElement;
use crate::parse::{Attribute, AttributeType, Category, ParsedElement};
use crate::{utils, Result};
use indoc::{formatdoc, writedoc};

pub fn generate(
    parsed: impl Iterator<Item = Result<MergedElement>>,
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
        let element_imports = filenames
            .iter()
            .map(|name| {
                format!(
                    "
                mod {name};\n
                pub use self::{name}::element::*;
            "
                )
            })
            .collect::<String>();

        let element_children = filenames
            .iter()
            .map(|name| {
                format!(
                    "
                pub use super::{name}::child::*;
            "
                )
            })
            .collect::<String>();

        let module = modules.iter().find(|el| &el.name == &dir).unwrap();
        let description = &module.description;
        let code = format!(
            "//! {description}
            {element_imports}
            
            /// The various child elements
            pub mod children {{
                {element_children}
            }}
            "
        );

        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code: utils::fmt(&code).expect("could not parse code"),
            dir,
        })
    }
    dirs.sort();

    // generate `elements/mod.rs` file
    let mods = dirs
        .iter()
        .map(|d| format!("pub mod {d};\n"))
        .collect::<String>();

    let all_files = dirs
        .iter()
        .map(|d| format!("pub(crate) use super::{d}::*;"))
        .collect::<String>();

    let code = format!(
        "//! HTML elements support
        {mods}

        /// All auto-generated items in this crate
        #[allow(unused)]
        pub(crate) mod all {{
            {all_files}
        }}
        "
    );
    output.push(CodeFile {
        filename: "mod.rs".to_owned(),
        code: utils::fmt(&code)?,
        dir: String::new(),
    });

    Ok(output)
}

/// Generate a single element.
fn generate_element(el: MergedElement) -> Result<CodeFile> {
    let dir = el.submodule_name.clone();
    let MergedElement {
        tag_name,
        struct_name,
        has_closing_tag,
        attributes,
        mdn_link,
        has_global_attributes,
        submodule_name,
        content_categories,
        dom_interface,
        permitted_child_elements,
    } = el;

    let filename = format!("{}.rs", tag_name);

    let categories = generate_categories(&content_categories, &struct_name);

    let children = match permitted_child_elements.len() {
        0 => String::new(),
        _ => format!("_children: Vec<super::child::{struct_name}Child>"),
    };

    let gen_children = match permitted_child_elements.len() {
        0 => String::new(),
        _ => "_children: vec![]".to_owned(),
    };

    let methods = gen_methods(&struct_name, &attributes);
    let html_element = gen_html_element(&struct_name, has_global_attributes);
    let children_enum = gen_enum(&struct_name, &permitted_child_elements);

    let sys_name = format!("html_sys::{submodule_name}::{struct_name}");

    let mut element = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        pub struct {struct_name} {{
            sys: {sys_name},
            {children}
        }}

        {methods}
        {html_element}
        {categories}

        impl std::convert::Into<{sys_name}> for {struct_name} {{
            fn into(self) -> {sys_name} {{
                self.sys
            }}
        }}

        impl From<{sys_name}> for {struct_name} {{
            fn from(sys: {sys_name}) -> Self {{
                Self {{
                    sys,
                    {gen_children}
                }}
            }}
        }}
    "#
    );

    let code = format!(
        "
        pub mod element {{
            {element}
        }}

        pub mod child {{
            {children_enum}
        }}
    "
    );

    Ok(CodeFile {
        filename,
        code: utils::fmt(&code)?,
        dir,
    })
}

fn gen_enum(struct_name: &str, permitted_child_elements: &[String]) -> String {
    if permitted_child_elements.len() == 0 {
        return String::new();
    }
    let members = permitted_child_elements
        .iter()
        .map(|el| {
            format!(
                "
            /// The {el} element
            {el}(crate::generated::all::{el}),
        "
            )
        })
        .collect::<String>();
    dbg!(format!(
        "
        /// The permitted child items for the `{struct_name}` element
        pub enum {struct_name}Child {{
            {members}
        }}"
    ))
}

fn gen_html_element(struct_name: &str, has_global_attributes: bool) -> String {
    if has_global_attributes {
        format!(
            "
            impl crate::HtmlElement for {struct_name} {{}}
        "
        )
    } else {
        String::new()
    }
}

fn generate_categories(categories: &[Category], struct_name: &str) -> String {
    let mut output = String::new();
    for cat in categories {
        generate_category(cat, &mut output, struct_name);
    }
    output
}

fn generate_category(cat: &Category, output: &mut String, struct_name: &str) {
    match cat {
        Category::Metadata => output.push_str(&format!(
            "impl crate::MetadataContent for {struct_name} {{}}"
        )),
        Category::Flow => {
            output.push_str(&format!("impl crate::FlowContent for {struct_name} {{}}"))
        }
        Category::Sectioning => {
            output.push_str(&format!(
                "impl crate::SectioningContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Heading => {
            output.push_str(&format!(
                "impl crate::HeadingContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Phrasing => {
            output.push_str(&format!(
                "impl crate::PhrasingContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Embedded => {
            output.push_str(&format!(
                "impl crate::EmbeddedContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Interactive => {
            output.push_str(&format!(
                "impl crate::InteractiveContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        Category::Palpable => output.push_str(&format!(
            "impl crate::PalpableContent for {struct_name} {{}}"
        )),
        Category::ScriptSupporting => output.push_str(&format!(
            "impl crate::ScriptSupportingContent for {struct_name} {{}}"
        )),
    }
}

fn format_content_model(cat: &Category) -> &'static str {
    match cat {
        Category::Metadata => "crate::MetadataContent",
        Category::Flow => "crate::FlowContent",
        Category::Flow => "crate::FlowContent",
        Category::Sectioning => "crate::SectioningContent",
        Category::Heading => "crate::HeadingContent",
        Category::Phrasing => "crate::PhrasingContent",
        Category::Embedded => "crate::EmbeddedContent",
        Category::Interactive => "crate::InteractiveContent",
        Category::Palpable => "crate::PalpableContent",
        Category::ScriptSupporting => "crate::ScriptSupportingContent",
    }
}

fn gen_methods(struct_name: &str, attributes: &[Attribute]) -> String {
    fn gen_method(attr: &Attribute) -> String {
        let name = &attr.name;
        let field_name = &attr.field_name;
        let return_ty = match &attr.ty {
            AttributeType::Bool => "bool".to_owned(),
            AttributeType::String => "std::option::Option<&str>".to_owned(),
            ty => format!("std::option::Option<{ty}>"),
        };

        let param_ty = match &attr.ty {
            AttributeType::Bool => "bool".to_owned(),
            ty => format!("std::option::Option<{ty}>"),
        };

        let field_access = match &attr.ty {
            AttributeType::Integer | AttributeType::Float | AttributeType::Bool => {
                format!("self.sys.{field_name}")
            }
            AttributeType::String => {
                format!("self.sys.{field_name}.as_deref()")
            }
            AttributeType::Identifier(_) => todo!(),
            AttributeType::Enumerable(_) => todo!(),
        };
        format!(
            "
            /// Get the value of the `{name}` attribute
            pub fn {field_name}(&self) -> {return_ty} {{
                {field_access}
            }}
            /// Set the value of the `{name}` attribute
            pub fn set_{field_name}(&mut self, value: {param_ty}) {{
                self.sys.{field_name} = value;
            }}",
        )
    }
    let methods: String = attributes.into_iter().map(gen_method).collect();

    match methods.len() {
        0 => String::new(),
        _ => format!(
            "
            impl {struct_name} {{
                {methods}
            }}
        "
        ),
    }
}
