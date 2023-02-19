use convert_case::{Case, Casing};
use std::{collections::HashSet, fmt::Display, fs, path::Path};

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

impl Display for AttributeTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeTy::Bool => write!(f, "bool"),
            AttributeTy::String => write!(f, "String"),
            AttributeTy::Integer => write!(f, "u64"),
            AttributeTy::Float => write!(f, "f64"),
            AttributeTy::Identifier(s) => write!(f, "{s}"),
        }
    }
}

type Database = HashSet<Definition>;

pub fn generate_html(path: &Path) -> Result<String> {
    let dir = fs::read_dir(path)?;

    let mut database = Database::default();
    let dir = dir.into_iter();
    // let dir = dir.take(1);
    for file in dir {
        let raw_data = fs::read_to_string(file?.path())?;
        let definitions = weedle::parse(&raw_data).map_err(|err| err.to_string())?;

        let definitions = definitions.into_iter();
        // let definitions = definitions.take(1);
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
    let mut output = String::new();
    for entry in database {
        output.push_str(&def_to_string(entry));
    }

    // missing types in the IDL files!
    output.push_str(
        "#[derive(Default, Debug, PartialEq, Clone)]
        pub struct FileList {}
        
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct EventTarget {}
        ",
    );
    Ok(output)
}

fn def_to_string(def: Definition) -> String {
    let Definition {
        name,
        inherits_from,
        members,
    } = def;

    let name = normalize_ident(&name);

    let (field, inherits) = match inherits_from {
        Some(from) => {
            let inherits = format!("impl ::std::ops::Deref for {name} {{\n    type Target = {from};\n    fn deref(&self) -> &Self::Target {{\n        &self.deref_target\n    }}\n}}");
            let field = format!("    deref_target: {from},");
            (field, inherits)
        }
        None => (String::new(), String::new()),
    };

    let mut fields = vec![field];
    let fields_iter = members.iter().filter_map(|member| match member.read_only {
        true => None,
        false => Some(format!(
            "    {}: {},",
            normalize_ident(&member.name.to_case(Case::Snake)),
            member.ty
        )),
    });
    fields.extend(fields_iter);
    let fields = fields.join("\n");

    let mut methods = vec![];
    let methods_iter = members.iter().filter_map(|member| match member.read_only {
        true => None,
        false => Some(format!(
            concat!(
                "    pub fn {name}(&self) -> {ty} {{\n        self.{name}.clone()\n    }}\n\n",
                "    pub fn set_{name}(&mut self, value: {ty}) {{\n        self.{name} = value;\n    }}\n"
            ),
            name = normalize_ident(&member.name.to_case(Case::Snake)),
            ty = member.ty
        )),
    });
    methods.extend(methods_iter);
    let methods = methods.join("\n");

    let derives = format!("#[derive(Default, Debug, PartialEq, Clone)]");
    let strukt = format!("{derives}\npub struct {name} {{\n{fields}\n}}\n");

    let impl_block = format!("impl {name} {{\n{methods}}}\n");

    format!("{strukt}\n{inherits}\n\n{impl_block}\n")
}

fn normalize_ident(s: &str) -> String {
    match &*s {
        "type" => "ty".to_owned(),
        "loop" => "loop_".to_owned(),
        s => s.to_owned(),
    }
}
