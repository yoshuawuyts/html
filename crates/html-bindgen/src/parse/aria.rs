use std::collections::{BTreeSet, HashMap};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use crate::{
    scrape::{ScrapedAriaElement, ScrapedAriaProperty, ScrapedAriaRole},
    Result,
};

use super::{Attribute, AttributeType};

/// The parsed WebIDL definitions converted from the raw spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedAriaRole {
    pub name: String,
    pub allowed_properties: Vec<String>,
}

/// The parsed WebIDL definitions converted from the raw spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedAriaProperty {
    pub name: String,
    pub idl_name: String,
    pub description: String,
    pub is_global: bool,
    pub value_kind: AriaPropertyValueKind,
    pub default_value: Option<String>,
}

impl From<ParsedAriaProperty> for Attribute {
    fn from(value: ParsedAriaProperty) -> Self {
        let field_name = value.idl_name.to_case(Case::Snake);
        Attribute {
            name: value.name,
            description: value.description,
            field_name,
            ty: value.value_kind.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AriaPropertyValueKind {
    Bool,
    Tristate,
    OptionalBool,
    IdRef,
    IdRefList,
    Integer,
    Number,
    String,
    Token(Vec<String>),
    TokenList(Vec<String>),
}

impl From<AriaPropertyValueKind> for AttributeType {
    fn from(value: AriaPropertyValueKind) -> Self {
        match value {
            AriaPropertyValueKind::Bool => AttributeType::Bool,
            AriaPropertyValueKind::Tristate => AttributeType::String, // TODO: AttributeType::Enumerable
            AriaPropertyValueKind::OptionalBool => AttributeType::Bool,
            AriaPropertyValueKind::IdRef => AttributeType::String, // TODO: AttributeType::Identifier
            AriaPropertyValueKind::IdRefList => AttributeType::String, // TODO: AttributeType::Vec?
            AriaPropertyValueKind::Integer => AttributeType::Integer,
            AriaPropertyValueKind::Number => AttributeType::Float,
            AriaPropertyValueKind::String => AttributeType::String,
            AriaPropertyValueKind::Token(_) => AttributeType::String, // TODO: AttributeType::Enumerable
            AriaPropertyValueKind::TokenList(_) => AttributeType::String, // TODO: AttributeType::Vec?
        }
    }
}

/// The parsed WebIDL definitions converted from the raw spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedAriaElement {
    pub tag_name: String,
    pub any_role: bool,
    pub no_role: bool,
    pub allowed_roles: BTreeSet<String>,
    pub global_aria_attributes: bool,
    pub no_aria_attributes: bool,
    pub allowed_aria_attributes: BTreeSet<String>,
    pub prohibited_aria_attributes: BTreeSet<String>,
}

pub fn parse_aria_roles(
    iter: impl Iterator<Item = Result<ScrapedAriaRole>>,
) -> Result<Vec<ParsedAriaRole>> {
    let mut output = vec![];
    for res in iter {
        let role = res?;

        if role.is_abstract {
            continue;
        }

        let allowed_properties = role
            .required
            .into_iter()
            .chain(role.inherited.into_iter())
            .chain(role.properties.into_iter())
            .collect();

        output.push(ParsedAriaRole {
            name: role.name,
            allowed_properties,
        });
    }

    Ok(output)
}

pub fn parse_aria_properties(
    iter: impl Iterator<Item = Result<ScrapedAriaProperty>>,
) -> Result<Vec<ParsedAriaProperty>> {
    let mut output = vec![];
    for res in iter {
        let property = res?;

        let default_value = property
            .values
            .iter()
            .filter_map(|x| x.strip_suffix(" (default)"))
            .next()
            .map(String::from);

        let value_kind = match property.value_kind.as_str() {
            "true/false" => AriaPropertyValueKind::Bool,
            "tristate" => AriaPropertyValueKind::Tristate,
            "true/false/undefined" => AriaPropertyValueKind::OptionalBool,
            "ID reference" => AriaPropertyValueKind::IdRef,
            "ID reference list" => AriaPropertyValueKind::IdRefList,
            "integer" => AriaPropertyValueKind::Integer,
            "number" => AriaPropertyValueKind::Number,
            "string" => AriaPropertyValueKind::String,
            "token" | "token list" => {
                let mut tokens = property.values;
                for t in &mut tokens {
                    if t.ends_with(" (default)") {
                        t.truncate(t.len() - " (default)".len());
                    }
                }

                if property.value_kind == "token" {
                    AriaPropertyValueKind::Token(tokens)
                } else {
                    AriaPropertyValueKind::TokenList(tokens)
                }
            }
            _ => panic!("Unexpected ARIA property value kind"),
        };

        let idl_name = match property.idl_name {
            Some(name) => name,
            None => {
                const MISSING_IDL_NAMES: [(&str, &str); 5] = [
                    ("aria-braillelabel", "ariaBrailleLabel"), // targeted for ARIA 1.3
                    ("aria-brailleroledescription", "ariaBrailleRoleDescription"), // targeted for ARIA 1.3
                    ("aria-dropeffect", "ariaDropEffect"), // deprecated in ARIA 1.1
                    ("aria-relevant", "ariaRelevant"),     // deprecated in ARIA 1.1
                    ("aria-grabbed", "ariaGrabbed"),       // deprecated in ARIA 1.1
                ];

                MISSING_IDL_NAMES
                    .iter()
                    .find(|x| x.0 == property.name)
                    .unwrap()
                    .1
                    .to_owned()
            }
        };

        output.push(ParsedAriaProperty {
            name: property.name,
            idl_name,
            description: property.description.unwrap_or_default(),
            is_global: property.is_global,
            value_kind,
            default_value,
        });
    }

    Ok(output)
}

pub fn parse_aria_elements(
    iter: impl Iterator<Item = Result<ScrapedAriaElement>>,
) -> Result<Vec<ParsedAriaElement>> {
    let mut output = HashMap::new();
    for res in iter {
        let el = res?;

        for tag_name in parse_tag_names(&el.name) {
            let mut new = false;
            let parsed = output.entry(tag_name.to_owned()).or_insert_with(|| {
                new = true;
                ParsedAriaElement {
                    tag_name: tag_name.to_owned(),
                    any_role: false,
                    no_role: true,
                    allowed_roles: BTreeSet::new(),
                    global_aria_attributes: false,
                    no_aria_attributes: true,
                    allowed_aria_attributes: BTreeSet::new(),
                    prohibited_aria_attributes: BTreeSet::new(),
                }
            });

            let any_role = el.links.iter().any(|x| x == "Any `role`")
                || el.links.iter().any(|x| x == "any `role`");
            let no_role = el.links.iter().any(|x| x == "No `role`")
                || el.links.iter().any(|x| x == "no `role`");
            let global_aria_attributes = el.global.is_some();
            let no_aria_attributes = el
                .strong
                .iter()
                .any(|x| x == "No `aria-*` attributes" || x == "No `role` or `aria-*` attributes");

            let mut allowed_roles =
                BTreeSet::from_iter(el.implicit_roles.iter().map(|x| x.replace('`', "")));
            allowed_roles.extend(el.allowed_roles.iter().map(|x| x.replace('`', "")));

            let allowed_aria_attributes =
                BTreeSet::from_iter(el.allowed_properties.iter().map(|x| x.replace('`', "")));
            let mut prohibited_aria_attributes =
                if el.links.iter().any(|x| x == "Naming Prohibited") {
                    BTreeSet::from([
                        "aria-braillelabel".to_owned(),
                        "aria-label".to_owned(),
                        "aria-labelledby".to_owned(),
                    ])
                } else {
                    BTreeSet::new()
                };

            // Special cases that can't be scraped
            if tag_name == "body" {
                // "...MUST NOT specify aria-hidden=true on the body element"
                prohibited_aria_attributes.insert("aria-hidden".to_owned());
            } else if tag_name == "dd" {
                // "...and any aria-* attributes applicable to the definition role"
                allowed_roles.insert("definition".to_owned());
            } else if tag_name == "footer" || tag_name == "header" {
                // "Naming Prohibited if exposed as generic."
                // (i.e. naming is not prohibited otherwise)
                prohibited_aria_attributes.remove("aria-braillelabel");
                prohibited_aria_attributes.remove("aria-label");
                prohibited_aria_attributes.remove("aria-labelledby");
            } else if tag_name == "input" {
                // extraneous conditional
                allowed_roles.remove("button if used with aria-pressed");
            }

            parsed.no_role &= no_role;
            parsed.any_role |= any_role;
            parsed.global_aria_attributes |= global_aria_attributes;
            parsed.no_aria_attributes &= no_aria_attributes || !allowed_aria_attributes.is_empty();
            parsed.allowed_roles.extend(allowed_roles.into_iter());
            parsed
                .allowed_aria_attributes
                .extend(allowed_aria_attributes.into_iter());

            if !new {
                prohibited_aria_attributes =
                    &parsed.prohibited_aria_attributes & &prohibited_aria_attributes;
            }
            parsed.prohibited_aria_attributes = prohibited_aria_attributes;
        }
    }

    Ok(output.into_values().collect())
}

fn parse_tag_names(name: &str) -> Vec<&str> {
    if name.starts_with("[^") {
        let end = name.find("^]").unwrap();
        vec![&name[2..end]]
    } else if name == "`h1 to h6`" {
        vec!["h1", "h2", "h3", "h4", "h5", "h6"]
    } else if name.starts_with("`input type=") {
        vec!["input"]
    } else {
        const IGNORED_ELEMENTS: [&str; 4] = [
            "`math`",
            "`SVG`",
            "form-associated custom element",
            "autonomous custom element",
        ];
        assert!(IGNORED_ELEMENTS.contains(&name));
        vec![]
    }
}
