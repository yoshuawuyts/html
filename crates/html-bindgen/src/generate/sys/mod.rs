use super::{CodeFile, Module};
use std::{collections::HashMap, iter};

use crate::merge::MergedElement;
use crate::parse::{Attribute, AttributeType};
use crate::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn includes() -> TokenStream {
    quote! {
        /// Render an element to a writer.
        pub trait RenderElement {
            /// Write the opening tag to a writer.
            fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result;

            /// Write the closing tag to a writer, if one is available.
            fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result;
        }

        /// Container for `data-*` attributes.
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct DataMap {
            map: std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>,
        }

        impl std::ops::Deref for DataMap {
            type Target =
                std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>;

            fn deref(&self) -> &Self::Target {
                &self.map
            }
        }

        impl std::ops::DerefMut for DataMap {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.map
            }
        }

        impl std::fmt::Display for DataMap {
            fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for (key, value) in self.map.iter() {
                    write!(writer, r#" data-{key}="{value}""#)?;
                }
                Ok(())
            }
        }
    }
}

pub fn generate(
    merged: impl Iterator<Item = Result<MergedElement>>,
    global_attributes: &[Attribute],
    modules: &[Module],
) -> Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut generated: HashMap<String, Vec<String>> = HashMap::new();

    // generate individual `{element}.rs` files
    for el in merged {
        let el = el?;
        let entry = generated.entry(el.submodule_name.clone());
        entry.or_default().push(el.tag_name.clone());
        let cf = generate_element(el)?;
        output.push(cf);
    }

    // generate `mod.rs` files
    let mut dirs = vec![];
    for (dir, mut filenames) in generated {
        filenames.sort();
        dirs.push(dir.clone());
        let code = filenames.into_iter().map(|name| {
            let name = format_ident!("{name}");
            quote! {
                mod #name;
                pub use #name::*;
            }
        });

        let module = modules.iter().find(|el| &el.name == &dir).unwrap();
        let description = format!(" {}", module.description);
        let code = quote! {
            #![doc = #description]
            #(#code)*
        };

        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code,
            dir,
        })
    }
    dirs.sort();

    // generate `lib.rs` file
    let code = dirs
        .into_iter()
        .map(|dir| format_ident!("{dir}"))
        .map(|dir| quote! { pub mod #dir; })
        .chain(iter::once(includes()))
        .chain(iter::once({
            let fields = generate_fields(global_attributes);
            let display_attrs = global_attributes.iter().map(generate_attribute_display);

            quote! {
                /// The "global attributes" struct
                #[derive(Debug, Clone, PartialEq, Default)]
                pub struct GlobalAttributes {
                    #fields
                }

                impl std::fmt::Display for GlobalAttributes {
                    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        #(#display_attrs)*
                        Ok(())
                    }
                }
            }
        }))
        .collect::<TokenStream>();
    output.push(CodeFile {
        filename: "lib.rs".to_owned(),
        code,
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
        ..
    } = el;

    let filename = format!("{tag_name}.rs");
    let struct_name = format_ident!("{struct_name}");
    let fields = generate_fields(&attributes);
    let opening_tag_content = generate_opening_tag(&attributes, &tag_name, has_global_attributes);
    let closing_tag_content = generate_closing_tag(&tag_name, has_closing_tag);

    let global_field = match has_global_attributes {
        true => quote! { global_attrs: crate::GlobalAttributes, },
        false => quote! {},
    };

    let deref_impls = if has_global_attributes {
        quote! {
            impl std::ops::Deref for #struct_name {
                type Target = crate::GlobalAttributes;

                fn deref(&self) -> &Self::Target {
                    &self.global_attrs
                }
            }

            impl std::ops::DerefMut for #struct_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.global_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let description =
        format!(" The HTML `<{tag_name}>` element\n\n [MDN Documentation]({mdn_link})");
    let code = quote! {
        #[doc = #description]
        #[doc(alias = #tag_name)]
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct #struct_name {
            pub data_map: crate::DataMap,
            #global_field
            #fields
        }

        impl crate::RenderElement for #struct_name {
            fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
                #opening_tag_content
                Ok(())
            }

            #[allow(unused_variables)]
            fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
                #closing_tag_content
                Ok(())
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use crate::RenderElement;
                self.write_opening_tag(writer)?;
                self.write_closing_tag(writer)?;
                Ok(())
            }
        }

        #deref_impls
    };

    Ok(CodeFile {
        filename,
        code,
        dir,
    })
}

fn generate_fields(attributes: &[Attribute]) -> TokenStream {
    let attributes = attributes.iter().map(|attr| {
        let description = format!(" {}", attr.description);
        let field_name = format_ident!("{}", attr.field_name);
        let ty = &attr.ty;

        match ty {
            AttributeType::Bool => quote! {
                #[doc = #description]
                pub #field_name: bool,
            },
            AttributeType::String => quote! {
                #[doc = #description]
                pub #field_name: std::option::Option<std::borrow::Cow<'static, str>>,
            },
            _ => quote! {
                #[doc = #description]
                pub #field_name: std::option::Option<#ty>,
            },
        }
    });

    quote! { #(#attributes)* }
}

fn generate_opening_tag(
    attributes: &[Attribute],
    tag_name: &str,
    has_global_attrs: bool,
) -> TokenStream {
    let preamble = match tag_name {
        "html" => "<!DOCTYPE html>",
        _ => "",
    };
    let attributes = attributes
        .iter()
        .map(|attr| generate_attribute_display(&attr))
        .chain(if has_global_attrs {
            Some(quote! { write!(writer, "{}", self.global_attrs)?; })
        } else {
            None
        });

    quote! {
        write!(writer, "{}<{}", #preamble, #tag_name)?;
        #(#attributes)*
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
    }
}

fn generate_closing_tag(tag_name: &str, has_closing_tag: bool) -> TokenStream {
    if has_closing_tag {
        quote! { write!(writer, "</{}>", #tag_name)?; }
    } else {
        quote! {}
    }
}

fn generate_attribute_display(attr: &Attribute) -> TokenStream {
    let Attribute {
        name,
        field_name,
        ty,
        ..
    } = &attr;
    let field_name = format_ident!("{field_name}");
    match ty {
        AttributeType::Bool => quote! {
            if self.#field_name {
                write!(writer, " {}", #name)?;
            }
        },
        AttributeType::String | AttributeType::Integer | AttributeType::Float => quote! {
            if let Some(field) = self.#field_name.as_ref() {
                write!(writer, r#" {}="{field}""#, #name)?;
            }
        },
        AttributeType::Identifier(_) => todo!(),
        AttributeType::Enumerable(_) => todo!(),
    }
}
