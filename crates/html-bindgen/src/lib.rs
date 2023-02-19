use std::{collections::HashSet, fs, path::Path};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

/// An HTML Node Definition
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Definition {
    name: String,
    inherits_from: Option<String>,
    members: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Attribute {
    name: String,
    read_only: bool,
    ty: AttributeTy,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AttributeTy {
    Bool,
    String,
    Integer,
    Float,
    Identifier(String),
}

type Database = HashSet<Definition>;

pub fn generate_html(path: &Path) -> Result<()> {
    let dir = fs::read_dir(path)?;

    let mut database = Database::default();
    let dir = dir.into_iter();
    let dir = dir.take(1);
    for file in dir {
        let raw_data = fs::read_to_string(file?.path())?;
        let definitions = weedle::parse(&raw_data).map_err(|err| err.to_string())?;

        let definitions = definitions.into_iter();
        let definitions = definitions.take(1);
        for def in definitions {
            match def {
                weedle::Definition::Callback(_) => {}
                weedle::Definition::CallbackInterface(_) => {}
                weedle::Definition::Interface(interface) => {
                    let name = interface.identifier.0.to_owned();
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
                                    name: format!("{}", attr.identifier.0),
                                    read_only: attr.readonly.is_some(),
                                    ty,
                                })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                    database.insert(Definition {
                        name,
                        inherits_from,
                        members,
                    });
                }
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
    dbg!(database);
    Ok(())
}
