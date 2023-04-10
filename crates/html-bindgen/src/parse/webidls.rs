use std::path::PathBuf;

use super::{Attribute, AttributeType};
use crate::Result;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use weedle::interface::InterfaceMember;

/// The parsed WebIDL definitions converted from the raw spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedInterface {
    pub name: String,
    pub inherits_from: Option<String>,
    pub attributes: Vec<Attribute>,
}

pub fn parse_webidls(
    iter: impl Iterator<Item = Result<(String, PathBuf)>>,
) -> Result<Vec<ParsedInterface>> {
    let mut outputs = vec![];
    for res in iter {
        let (string, path) = res?;
        let filename = path.file_name().unwrap().to_str().unwrap();
        if !filename.starts_with("HTML") {
            continue;
        }
        let string = string.trim();
        let definitions = weedle::parse(&string).map_err(|err| err.to_string())?;
        let definitions = definitions.into_iter();
        for def in definitions {
            if let weedle::Definition::Interface(interface) = def {
                outputs.push(ParsedInterface {
                    name: parse_interface_name(&interface),
                    inherits_from: parse_inheritance(&interface),
                    attributes: interface
                        .members
                        .body
                        .iter()
                        .filter_map(parse_attributes)
                        .collect::<Vec<_>>(),
                });
            }
        }
    }
    Ok(outputs)
}

fn parse_interface_name(interface: &weedle::InterfaceDefinition) -> String {
    interface.identifier.0.to_owned()
}

fn parse_inheritance(interface: &weedle::InterfaceDefinition) -> Option<String> {
    interface
        .inheritance
        .map(|parent| parent.identifier.0.to_string())
}

fn parse_attributes(member: &InterfaceMember) -> Option<Attribute> {
    if let InterfaceMember::Attribute(attr) = member {
        // NOTE: we're skipping over all DOM-only methods for now.
        if attr.readonly.is_some() {
            return None;
        }

        let ty = match &attr.type_.type_ {
            weedle::types::Type::Single(ty) => match ty {
                weedle::types::SingleType::NonAny(ty) => match ty {
                    weedle::types::NonAnyType::Integer(_) => AttributeType::Integer,
                    weedle::types::NonAnyType::FloatingPoint(_) => AttributeType::Float,
                    weedle::types::NonAnyType::Boolean(_) => AttributeType::Bool,
                    weedle::types::NonAnyType::Object(_) => {
                        // `js-sys` doesn't handle this either, so we just skip right past it.
                        return None;
                    }
                    weedle::types::NonAnyType::USVString(_)
                    | weedle::types::NonAnyType::DOMString(_) => AttributeType::String,
                    weedle::types::NonAnyType::Identifier(id) => {
                        AttributeType::Identifier(id.type_.0.to_owned())
                    }
                    ty => unreachable!("{ty:?} is not a recognized type"),
                },
                weedle::types::SingleType::Any(_) => return None,
            },
            _ => return None,
        };
        let field_name = attr.identifier.0.to_string().to_case(Case::Snake);
        let field_name = super::normalize_field_name(&field_name);
        Some(Attribute {
            name: attr.identifier.0.to_string(),
            field_name,
            description: String::new(),
            ty,
        })
    } else {
        None
    }
}
