use super::types::*;
use super::Database;
use indoc::formatdoc;

use std::io;

pub(crate) fn parse_webidl(
    iter: impl Iterator<Item = io::Result<String>>,
) -> Result<super::Database> {
    let mut database = Database::default();
    for string in iter {
        let string = string?;
        let definitions = weedle::parse(&string).map_err(|err| err.to_string())?;

        let definitions = definitions.into_iter();
        // let definitions = definitions.take(1);
        for def in definitions {
            match def {
                weedle::Definition::Interface(interface) => {
                    let dom_interface = interface.identifier.0.to_owned();
                    let inherits_from = interface
                        .inheritance
                        .map(|parent| parent.identifier.0.to_string());

                    let members = interface
                        .members
                        .body
                        .into_iter()
                        .filter_map(|member| match member {
                            weedle::interface::InterfaceMember::Attribute(attr) => {
                                let ty = match attr.type_.type_ {
                                    weedle::types::Type::Single(ty) => match ty {
                                        weedle::types::SingleType::NonAny(ty) => match ty {
                                            weedle::types::NonAnyType::Integer(_) => {
                                                AttributeTy::Integer
                                            }
                                            weedle::types::NonAnyType::FloatingPoint(_) => {
                                                AttributeTy::Float
                                            }
                                            weedle::types::NonAnyType::Boolean(_) => {
                                                AttributeTy::Bool
                                            }
                                            weedle::types::NonAnyType::Object(_) => {
                                                // `js-sys` doesn't handle this either, so we just skip right past it.
                                                return None;
                                            }
                                            weedle::types::NonAnyType::USVString(_)
                                            | weedle::types::NonAnyType::DOMString(_) => {
                                                AttributeTy::String
                                            }
                                            weedle::types::NonAnyType::Identifier(id) => {
                                                AttributeTy::Identifier(id.type_.0.to_owned())
                                            }
                                            ty => unreachable!("{ty:?} is not a recognized type"),
                                        },
                                        weedle::types::SingleType::Any(_) => {
                                            unreachable!("found `Any` type")
                                        }
                                    },
                                    _ => unreachable!("found union type"),
                                };
                                Some(Attribute {
                                    name: formatdoc!("{}", attr.identifier.0),
                                    read_only: attr.readonly.is_some(),
                                    ty,
                                })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                    database.insert(Definition {
                        tag_name: convert_tag_name(&dom_interface).to_string(),
                        dom_interface,
                        inherits_from,
                        members,
                    });
                }
                weedle::Definition::Callback(_) => {}
                weedle::Definition::CallbackInterface(_) => {}
                weedle::Definition::InterfaceMixin(_) => {}
                weedle::Definition::Namespace(_) => {}
                weedle::Definition::Dictionary(_) => {}
                weedle::Definition::PartialInterface(_) => {}
                weedle::Definition::PartialInterfaceMixin(_) => {}
                weedle::Definition::PartialDictionary(_) => {}
                weedle::Definition::PartialNamespace(_) => {}
                weedle::Definition::Enum(_) => {}
                weedle::Definition::Typedef(_) => {}
                weedle::Definition::IncludesStatement(_) => {}
                weedle::Definition::Implements(_) => {}
            }
        }
    }
    Ok(database)
}

fn convert_tag_name(s: &str) -> &str {
    match s {
        "tablerow" => "tr",
        "tablecaption" => "caption",
        "tablesection" => "tbody",
        "tablecol" => "col",
        s => s,
    }
}
