use super::{CodeFile, ModuleMapping};
use crate::merge::{MergedCategory, MergedElement};
use crate::parse::{Attribute, AttributeType};
use crate::{utils, Result};
use builder::gen_builder;
use indoc::formatdoc;

mod builder;

pub fn generate(
    parsed: impl Iterator<Item = Result<MergedElement>>,
    global_attributes: &[Attribute],
    module_map: &[super::ModuleMapping],
) -> Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut tag_names = vec![];

    // generate individual `{element}.rs` files
    for el in parsed {
        let el = el?;
        tag_names.push(el.tag_name.clone());
        output.push(generate_element(el, global_attributes)?);
    }

    let mods = tag_names
        .iter()
        .map(|tag_name| format!("pub(crate) mod {tag_name};"))
        .collect::<String>();

    let all_files = tag_names
        .iter()
        .map(|tag_name| format!("pub(crate) use crate::generated::{tag_name}::element::*;"))
        .collect::<String>();

    let all_builders = tag_names
        .iter()
        .map(|tag_name| format!("pub(crate) use crate::generated::{tag_name}::builder::*;"))
        .collect::<String>();

    let by_mapping = module_map
        .iter()
        .map(|ModuleMapping { name, children }| {
            let elements = children
                .iter()
                .map(|tag_name| format!("pub use crate::generated::{tag_name}::element::*;"))
                .collect::<String>();
            let builders = children
                .iter()
                .map(|tag_name| format!("pub use crate::generated::{tag_name}::builder::*;"))
                .collect::<String>();
            let children = children
                .iter()
                .map(|tag_name| format!("pub use crate::generated::{tag_name}::child::*;"))
                .collect::<String>();

            format!(
                "
                pub mod {name} {{
                    pub mod elements {{
                        {elements}
                    }}
                    /// Child elements
                    pub mod children {{
                        {children}
                    }}
                    /// Element builders
                    pub mod builders {{
                        {builders}
                    }}
                }}
            "
            )
        })
        .collect::<String>();

    let code = format!(
        "//! HTML elements support
        {mods}

        /// All auto-generated items in this crate
        #[allow(unused)]
        pub(crate) mod all {{
            {all_files}

            /// All auto-generated builders
            pub mod builders {{
                {all_builders}
            }}
        }}

        /// Modules according to the MDN mappings.
        pub(crate) mod mdn {{
            {by_mapping}
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
fn generate_element(el: MergedElement, global_attributes: &[Attribute]) -> Result<CodeFile> {
    let MergedElement {
        tag_name,
        struct_name,
        attributes,
        mdn_link,
        has_global_attributes,
        submodule_name,
        content_categories,
        permitted_child_elements,
        ..
    } = el;

    let filename = format!("{}.rs", tag_name);
    let enum_name = format!("super::child::{struct_name}Child");
    let sys_name = format!("html_sys::{submodule_name}::{struct_name}");

    let has_children = permitted_child_elements.len() != 0;
    let categories_impl = gen_categories_impl(&content_categories, &struct_name);
    let html_element_impl = gen_html_element_impl(&struct_name, has_global_attributes);
    let children_enum = gen_enum(&struct_name, &permitted_child_elements);
    let child_methods = gen_child_methods(&struct_name, &enum_name, &permitted_child_elements);
    let display_impl = gen_display_impl(&struct_name, has_children);

    let method_attributes = match has_global_attributes {
        true => {
            let mut attrs = attributes.clone();
            attrs.append(&mut global_attributes.to_owned());
            attrs
        }
        false => attributes.clone(),
    };
    let builder = gen_builder(&struct_name, &permitted_child_elements, &method_attributes);
    let getter_setter_methods = gen_methods(&struct_name, &method_attributes);

    let children = match has_children {
        true => format!("children: Vec<{enum_name}>"),
        false => String::new(),
    };

    let gen_children = match has_children {
        true => "children: vec![]".to_owned(),
        false => String::new(),
    };

    let element = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
        pub struct {struct_name} {{
            sys: {sys_name},
            {children}
        }}

        impl {struct_name} {{
            /// Create a new builder
            pub fn builder() -> super::builder::{struct_name}Builder {{
                super::builder::{struct_name}Builder::new(Default::default())
            }}
        }}

        {getter_setter_methods}
        {child_methods}

        {display_impl}
        {html_element_impl}
        {categories_impl}

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

        pub mod builder {{
            {builder}
        }}
    "
    );

    Ok(CodeFile {
        filename,
        code: utils::fmt(&code)?,
        dir: "".to_owned(),
    })
}

fn gen_display_impl(struct_name: &str, has_children: bool) -> String {
    let write_children = match has_children {
        true => format!(
            "for el in &self.children {{
                std::fmt::Display::fmt(&el, f)?;
            }}"
        ),
        false => String::new(),
    };
    format!(
        "
        impl std::fmt::Display for {struct_name} {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
                {write_children}
                html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
                Ok(())
            }}
        }}
    "
    )
}

fn gen_child_methods(
    struct_name: &str,
    enum_name: &str,
    permitted_child_elements: &[String],
) -> String {
    if permitted_child_elements.len() == 0 {
        return String::new();
    }

    format!(
        "impl {struct_name} {{
            /// Access the element's children
            pub fn children(&self) -> &[{enum_name}] {{
                self.children.as_ref()
            }}

            /// Mutably access the element's children
            pub fn children_mut(&mut self) -> &mut Vec<{enum_name}> {{
                &mut self.children
            }}
        }}"
    )
}

fn gen_enum(struct_name: &str, permitted_child_elements: &[String]) -> String {
    if permitted_child_elements.len() == 0 {
        return String::new();
    }

    /// Take an element type, and convert it to a path
    /// In the case of the special `Text` type, we use a
    /// Rust `String`.
    fn gen_ty_path(el: &str) -> String {
        if el == "Text" {
            "std::borrow::Cow<'static, str>".to_owned()
        } else {
            format!("crate::generated::all::{el}")
        }
    }

    let members = permitted_child_elements
        .iter()
        .map(|el| {
            let ty = gen_ty_path(el);
            format!(
                "/// The {el} element
                {el}({ty}),"
            )
        })
        .collect::<String>();

    let from = permitted_child_elements
        .iter()
        .map(|el| {
            let ty = gen_ty_path(el);

            let base_impl = format!(
                "
            impl std::convert::From<{ty}> for {struct_name}Child {{
                fn from(value: {ty}) -> Self {{
                    Self::{el}(value)
                }}
            }}
        "
            );
            if ty.contains("borrow") {
                format!(
                    "
                    {base_impl}

                    impl std::convert::From<&'static str> for {struct_name}Child {{
                        fn from(value: &'static str) -> Self {{
                            Self::{el}(value.into())
                        }}
                    }}
                    impl std::convert::From<String> for {struct_name}Child {{
                        fn from(value: String) -> Self {{
                            Self::{el}(value.into())
                        }}
                    }}
                "
                )
            } else {
                base_impl
            }
        })
        .collect::<String>();

    let display_patterns = permitted_child_elements
        .iter()
        .map(|el| format!(r#"Self::{el}(el) => write!(f, "{{el}}"),"#))
        .collect::<String>();

    format!(
        "
        /// The permitted child items for the `{struct_name}` element
        #[derive(Debug, PartialEq, PartialOrd, Clone)]
        pub enum {struct_name}Child {{
            {members}
        }}
        {from}

        impl std::fmt::Display for {struct_name}Child {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                match self {{
                    {display_patterns}
                }}
            }}
        }}
        "
    )
}

fn gen_html_element_impl(struct_name: &str, has_global_attributes: bool) -> String {
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

fn gen_categories_impl(categories: &[MergedCategory], struct_name: &str) -> String {
    let mut output = String::new();
    for cat in categories {
        generate_category(cat, &mut output, struct_name);
    }
    output
}

fn generate_category(cat: &MergedCategory, output: &mut String, struct_name: &str) {
    match cat {
        MergedCategory::Metadata => output.push_str(&format!(
            "impl crate::MetadataContent for {struct_name} {{}}"
        )),
        MergedCategory::Flow => {
            output.push_str(&format!("impl crate::FlowContent for {struct_name} {{}}"))
        }
        MergedCategory::Sectioning => {
            output.push_str(&format!(
                "impl crate::SectioningContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        MergedCategory::Heading => {
            output.push_str(&format!(
                "impl crate::HeadingContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        MergedCategory::Phrasing => {
            output.push_str(&format!(
                "impl crate::PhrasingContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        MergedCategory::Embedded => {
            output.push_str(&format!(
                "impl crate::EmbeddedContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        MergedCategory::Interactive => {
            output.push_str(&format!(
                "impl crate::InteractiveContent for {struct_name} {{}}"
            ));
            // generate_category(&Category::Flow, output, struct_name);
        }
        MergedCategory::Palpable => output.push_str(&format!(
            "impl crate::PalpableContent for {struct_name} {{}}"
        )),
        MergedCategory::ScriptSupporting => output.push_str(&format!(
            "impl crate::ScriptSupportingContent for {struct_name} {{}}"
        )),
        MergedCategory::Transparent => output.push_str(&format!(
            "impl crate::TransparentContent for {struct_name} {{}}"
        )),
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
            AttributeType::String => {
                "std::option::Option<impl Into<std::borrow::Cow<'static, str>>>".to_owned()
            }
            ty => format!("std::option::Option<{ty}>"),
        };

        let field_access = match &attr.ty {
            AttributeType::Integer | AttributeType::Float | AttributeType::Bool => {
                format!("self.sys.{field_name}")
            }
            AttributeType::String => {
                format!("self.sys.{field_name}.as_deref()")
            }
            _ => todo!("unhandled type"),
        };
        let field_setter = match &attr.ty {
            AttributeType::String => format!("value.map(|v| v.into())"),
            _ => format!("value"),
        };
        format!(
            "
            /// Get the value of the `{name}` attribute
            pub fn {field_name}(&self) -> {return_ty} {{
                {field_access}
            }}
            /// Set the value of the `{name}` attribute
            pub fn set_{field_name}(&mut self, value: {param_ty}) {{
                self.sys.{field_name} = {field_setter};
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
