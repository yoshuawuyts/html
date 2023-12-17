use super::{CodeFile, ModuleMapping};
use crate::merge::{MergedCategory, MergedElement};
use crate::parse::{Attribute, AttributeType};
use crate::Result;
use builder::gen_builder;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

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
        tag_names.push(format_ident!("{}", el.tag_name));
        output.push(generate_element(el, global_attributes)?);
    }
    tag_names.sort();

    let mods = tag_names
        .iter()
        .map(|tag_name| quote! { pub(crate) mod #tag_name; });

    let all_files = tag_names
        .iter()
        .map(|tag_name| quote! { pub(crate) use crate::generated::#tag_name::element::*; });

    let all_builders = tag_names
        .iter()
        .map(|tag_name| quote! { pub(crate) use crate::generated::#tag_name::builder::*; });

    let all_children = tag_names
        .iter()
        .map(|tag_name| quote! { pub(crate) use crate::generated::#tag_name::child::*; });

    let by_mapping = module_map.iter().map(|ModuleMapping { name, children }| {
        let name = format_ident!("{name}");
        let children = children.iter().map(|tag_name| format_ident!("{tag_name}"));
        let elements = children
            .clone()
            .map(|tag_name| quote! { pub use crate::generated::#tag_name::element::*; });
        let builders = children
            .clone()
            .map(|tag_name| quote! { pub use crate::generated::#tag_name::builder::*; });
        let children = children
            .clone()
            .map(|tag_name| quote! { pub use crate::generated::#tag_name::child::*; });

        quote! {
            pub mod #name {
                pub mod elements {
                    #(#elements)*
                }
                /// Child elements
                pub mod children {
                    #(#children)*
                }
                /// Element builders
                pub mod builders {
                    #(#builders)*
                }
            }
        }
    });

    let code = quote! {
        //! HTML elements support
        #(#mods)*

        /// All auto-generated items in this crate
        #[allow(unused)]
        pub(crate) mod all {
            #(#all_files)*

            /// All auto-generated builders
            pub mod builders {
                #(#all_builders)*
            }

            /// All auto-generated children
            pub mod children {
                #(#all_children)*
            }
        }

        /// Modules according to the MDN mappings.
        pub(crate) mod mdn {
            #(#by_mapping)*
        }
    };
    output.push(CodeFile {
        filename: "mod.rs".to_owned(),
        code,
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
        has_closing_tag,
        ..
    } = el;

    let filename = format!("{tag_name}.rs");
    let struct_name = format_ident!("{struct_name}");
    let child_name = format_ident!("{struct_name}Child");
    let submodule_name = format_ident!("{submodule_name}");
    let enum_name = quote! { super::child::#child_name };
    let sys_name = quote! { html_sys::#submodule_name::#struct_name };
    let builder_name = format_ident!("{struct_name}Builder");
    let builder_name = quote! { super::builder::#builder_name };

    let should_indent = match tag_name.as_str() {
        "pre" => false,
        _ => true,
    };

    let has_children = permitted_child_elements.len() != 0;
    let categories_impl = gen_categories_impl(&content_categories, &struct_name);
    let html_element_impl = gen_html_element_impl(&struct_name, has_global_attributes);
    let children_enum = gen_enum(&struct_name, &permitted_child_elements, should_indent);
    let child_methods = gen_child_methods(&struct_name, &enum_name, &permitted_child_elements);
    let data_map_methods = gen_data_map_methods(&struct_name);
    let display_impl = gen_fmt_impl(&struct_name, has_children, has_closing_tag, should_indent);

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
        true => quote! { children: Vec<#enum_name> },
        false => quote! {},
    };

    let gen_children = match has_children {
        true => quote! { children: vec![] },
        false => quote! {},
    };

    let description =
        format!(" The HTML `<{tag_name}>` element\n\n [MDN Documentation]({mdn_link})");
    let struct_name = format_ident!("{struct_name}");
    let element = quote! {
        #[doc = #description]
        #[doc(alias = #tag_name)]
        #[non_exhaustive]
        #[derive(PartialEq, Clone, Default)]
        pub struct #struct_name {
            sys: #sys_name,
            #children
        }

        impl #struct_name {
            /// Create a new builder
            pub fn builder() -> #builder_name {
                #builder_name::new(Default::default())
            }
        }

        #data_map_methods
        #getter_setter_methods
        #child_methods

        #display_impl
        #html_element_impl
        #categories_impl

        impl std::convert::Into<#sys_name> for #struct_name {
            fn into(self) -> #sys_name {
                self.sys
            }
        }

        impl From<#sys_name> for #struct_name {
            fn from(sys: #sys_name) -> Self {
                Self {
                    sys,
                    #gen_children
                }
            }
        }
    };

    let code = quote! {
        pub mod element {
            #element
        }

        pub mod child {
            #children_enum
        }

        pub mod builder {
            #builder
        }
    };

    Ok(CodeFile {
        filename,
        code,
        dir: "".to_owned(),
    })
}

fn gen_fmt_impl(
    struct_name: &Ident,
    has_children: bool,
    has_closing_tag: bool,
    should_indent: bool,
) -> TokenStream {
    let write_debug_children = if has_children && should_indent {
        quote! {
            if !self.children.is_empty() {
                write!(f, "\n")?;
            }
            for el in &self.children {
                crate::Render::render(&el, f, depth)?;
                write!(f, "\n")?;
            }
        }
    } else if has_children && !should_indent {
        quote! {
            for el in &self.children {
                crate::Render::render(&el, f, 0)?;
            }
        }
    } else {
        quote! {}
    };

    let write_display_children = if has_children {
        quote! {
            for el in &self.children {
                write!(f, "{el}")?;
            }
        }
    } else {
        quote! {}
    };

    let write_closing_tag = if has_closing_tag && should_indent {
        quote! {
            write!(f, "{:level$}", "", level = depth * 4)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
        }
    } else if has_closing_tag && !should_indent {
        quote! {
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
        }
    } else {
        quote! {}
    };
    quote! {
        impl crate::Render for #struct_name {
            fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
                write!(f, "{:level$}", "", level = depth * 4)?;
                html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
                #write_debug_children
                #write_closing_tag
                Ok(())
            }
        }

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                crate::Render::render(self, f, 0)?;
                Ok(())
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
                #write_display_children
                html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
                Ok(())
            }
        }
    }
}

fn gen_child_methods(
    struct_name: &Ident,
    enum_name: &TokenStream,
    permitted_child_elements: &[String],
) -> TokenStream {
    if permitted_child_elements.len() == 0 {
        return quote! {};
    }

    quote! {
        impl #struct_name {
            /// Access the element's children
            pub fn children(&self) -> &[#enum_name] {
                self.children.as_ref()
            }

            /// Mutably access the element's children
            pub fn children_mut(&mut self) -> &mut Vec<#enum_name> {
                &mut self.children
            }
        }
    }
}

fn gen_data_map_methods(struct_name: &Ident) -> TokenStream {
    quote! {
        impl #struct_name {
            /// Access the element's `data-*` properties
            pub fn data_map(&self) -> &html_sys::DataMap {
                &self.sys.data_map
            }

            /// Mutably access the element's `data-*` properties
            pub fn data_map_mut(&mut self) -> &mut html_sys::DataMap {
                &mut self.sys.data_map
            }
        }
    }
}

fn gen_enum(
    struct_name: &Ident,
    permitted_child_elements: &[String],
    should_indent: bool,
) -> TokenStream {
    if permitted_child_elements.len() == 0 {
        return quote! {};
    }

    /// Take an element type, and convert it to a path
    /// In the case of the special `Text` type, we use a
    /// Rust `String`.
    fn gen_ty_path(el: &str) -> TokenStream {
        if el == "Text" {
            quote! { std::borrow::Cow<'static, str> }
        } else {
            let el = format_ident!("{el}");
            quote! { crate::generated::all::#el }
        }
    }

    let child_name = format_ident!("{struct_name}Child");
    let members = permitted_child_elements.iter().map(|el| {
        let ty = gen_ty_path(el);
        let el = format_ident!("{el}");
        let description = format!(" The {el} element");
        quote! {
            #[doc = #description]
            #el(#ty),
        }
    });

    let from = permitted_child_elements.iter().map(|el| {
        let ty = gen_ty_path(el);
        let el = format_ident!("{el}");

        let base_impl = quote! {
            impl std::convert::From<#ty> for #child_name {
                fn from(value: #ty) -> Self {
                    Self::#el(value)
                }
            }
        };
        if el == "Text" {
            quote! {
                #base_impl

                impl std::convert::From<&'static str> for #child_name {
                    fn from(value: &'static str) -> Self {
                        Self::#el(value.into())
                    }
                }
                impl std::convert::From<String> for #child_name {
                    fn from(value: String) -> Self {
                        Self::#el(value.into())
                    }
                }
            }
        } else {
            base_impl
        }
    });

    let increase_depth = match should_indent {
        true => quote! { + 1 },
        false => quote! {},
    };
    let debug_patterns = permitted_child_elements
        .iter()
        .map(|el| format_ident!("{el}"))
        .map(|el| {
            quote! { Self::#el(el) => crate::Render::render(el, f, depth #increase_depth), }
        });
    let display_patterns = permitted_child_elements
        .iter()
        .map(|el| format_ident!("{el}"))
        .map(|el| quote! { Self::#el(el) => write!(f, "{el}"), });
    let description = format!(" The permitted child items for the `{struct_name}` element");

    quote! {
        #[doc = #description]
        #[derive(PartialEq, Clone)]
        pub enum #child_name {
            #(#members)*
        }
        #(#from)*

        impl crate::Render for #child_name {
            fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
                match self {
                    #(#debug_patterns)*
                }
            }
        }

        impl std::fmt::Debug for #child_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                crate::Render::render(self, f, 0)?;
                Ok(())
            }
        }

        impl std::fmt::Display for #child_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_patterns)*
                }
            }
        }
    }
}

fn gen_html_element_impl(struct_name: &Ident, has_global_attributes: bool) -> TokenStream {
    if has_global_attributes {
        quote! { impl crate::HtmlElement for #struct_name {} }
    } else {
        quote! {}
    }
}

fn gen_categories_impl(categories: &[MergedCategory], struct_name: &Ident) -> TokenStream {
    categories
        .iter()
        .map(|cat| match cat {
            MergedCategory::Metadata => {
                quote! { impl crate::MetadataContent for #struct_name {} }
            }
            MergedCategory::Flow => {
                quote! { impl crate::FlowContent for #struct_name {} }
            }
            MergedCategory::Sectioning => {
                quote! { impl crate::SectioningContent for #struct_name {} }
            }
            MergedCategory::Heading => {
                quote! { impl crate::HeadingContent for #struct_name {} }
            }
            MergedCategory::Phrasing => {
                quote! { impl crate::PhrasingContent for #struct_name {} }
            }
            MergedCategory::Embedded => {
                quote! { impl crate::EmbeddedContent for #struct_name {} }
            }
            MergedCategory::Interactive => {
                quote! { impl crate::InteractiveContent for #struct_name {} }
            }
            MergedCategory::Palpable => {
                quote! { impl crate::PalpableContent for #struct_name {} }
            }
            MergedCategory::ScriptSupporting => {
                quote! { impl crate::ScriptSupportingContent for #struct_name {} }
            }
            MergedCategory::Transparent => {
                quote! { impl crate::TransparentContent for #struct_name {} }
            }
        })
        .collect()
}

fn gen_methods(struct_name: &Ident, attributes: &[Attribute]) -> TokenStream {
    fn gen_method(attr: &Attribute) -> TokenStream {
        let name = &attr.name;
        let field_name = format_ident!("{}", attr.field_name);
        let return_ty = match &attr.ty {
            AttributeType::Bool => quote! { bool },
            AttributeType::String => quote! { std::option::Option<&str> },
            ty => quote! { std::option::Option<#ty> },
        };

        let param_ty = match &attr.ty {
            AttributeType::Bool => quote! { bool },
            AttributeType::String => {
                quote! { std::option::Option<impl Into<std::borrow::Cow<'static, str>>> }
            }
            ty => quote! { std::option::Option<#ty> },
        };

        let field_access = match &attr.ty {
            AttributeType::Integer | AttributeType::Float | AttributeType::Bool => {
                quote! { self.sys.#field_name }
            }
            AttributeType::String => {
                quote! { self.sys.#field_name.as_deref() }
            }
            _ => todo!("unhandled type"),
        };
        let field_setter = match &attr.ty {
            AttributeType::String => quote! { value.map(|v| v.into()) },
            _ => quote! { value },
        };
        let getter = format_ident!("{field_name}");
        let setter = format_ident!("set_{field_name}");
        let getter_description = format!(" Get the value of the `{name}` attribute");
        let setter_description = format!(" Set the value of the `{name}` attribute");
        quote! {
            #[doc = #getter_description]
            pub fn #getter(&self) -> #return_ty {
                #field_access
            }

            #[doc = #setter_description]
            pub fn #setter(&mut self, value: #param_ty) {
                self.sys.#field_name = #field_setter;
            }
        }
    }
    let methods: Vec<_> = attributes.into_iter().map(gen_method).collect();

    match methods.len() {
        0 => quote! {},
        _ => quote! {
            impl #struct_name {
                #(#methods)*
            }
        },
    }
}
