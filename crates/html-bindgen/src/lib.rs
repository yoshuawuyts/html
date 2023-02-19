use convert_case::{Case, Casing};
use indoc::formatdoc;
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
                                    name: formatdoc!("{}", attr.identifier.0),
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
    output.push_str(
        "/// An HTML Element
        pub trait HtmlElement: ::std::fmt::Display {}\n
        ",
    );
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

    let is_element = name.starts_with("HTML") && name.ends_with("Element") && name != "HTMLElement";

    let struct_ident = match is_element {
        true => format!("{name}<T>"),
        false => format!("{name}"),
    };

    let impl_ident = match is_element {
        true => "impl <T: HtmlElement>",
        false => "impl",
    };

    let (field, inherits) = match inherits_from {
        Some(from) => {
            let inherits = formatdoc!(
                "{impl_ident} ::std::ops::Deref for {struct_ident} {{
                    type Target = {from};
                    fn deref(&self) -> &Self::Target {{
                        &self.deref_target
                    }}
                }}"
            );
            let field = formatdoc!("deref_target: {from},");
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
        false => Some(formatdoc!(
            "pub fn {name}(&self) -> {ty} {{
                self.{name}.clone()
            }}

            pub fn set_{name}(&mut self, value: {ty}) {{
                self.{name} = value;
            }}",
            name = normalize_ident(&member.name.to_case(Case::Snake)),
            ty = member.ty
        )),
    });
    methods.extend(methods_iter);
    let methods = methods.join("\n");

    let strukt = formatdoc!(
        "
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct {struct_ident} {{
            {fields}
        }}
    "
    );
    let inherent_impl = formatdoc!(
        "
        {impl_ident} {struct_ident} {{
            {methods}
        }}
    "
    );

    // If we're dealing with an HTML element, implement Display + HtmlElement
    if is_element {
        let tag_name = name
            .strip_prefix("HTML")
            .unwrap()
            .strip_suffix("Element")
            .unwrap()
            .to_lowercase();
        let tag_name = convert_tag_name(&tag_name);
        let html_impl = formatdoc!("{impl_ident} HtmlElement for {struct_ident} {{}}\n");
        let display_impl = formatdoc!(
            "{impl_ident} ::std::fmt::Display for {struct_ident} {{
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
                    write!(f, \"<{tag_name}>\")?;
                    write!(f, \"</{tag_name}>\")?;
                    Ok(())
                }}
            }}\n"
        );
        formatdoc!("{strukt}\n{inherits}\n\n{inherent_impl}\n{html_impl}\n{display_impl}")
    } else {
        formatdoc!("{strukt}\n{inherits}\n\n{inherent_impl}\n")
    }
}

fn normalize_ident(s: &str) -> String {
    match &*s {
        "type" => "ty".to_owned(),
        "loop" => "loop_".to_owned(),
        s => s.to_owned(),
    }
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
