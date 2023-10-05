use std::collections::HashMap;

use scraper::ElementRef;

use crate::Result;

/// The raw role values extracted from the WAI-ARIA spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScrapedAriaRole {
    pub name: String,

    /// Implicit value for role
    pub implicit_values: Option<String>,
    pub is_abstract: bool,
    /// Base concept
    pub base: Option<String>,
    pub are_children_presentational: bool,
    /// Subclass roles
    pub children: Vec<String>,
    /// Prohibited states and properties
    pub disallowed: Vec<String>,
    /// Inherited states and properties
    pub inherited: Vec<String>,
    /// Allowed accessibility child roles
    pub must_contain: Vec<String>,
    pub name_from: Option<String>,
    pub is_name_required: bool,
    /// Superclass role
    pub parent: Vec<String>,
    /// Supported states and properties
    pub properties: Vec<String>,
    /// Related concepts
    pub related: Option<String>,
    /// Required states and properties
    pub required: Vec<String>,
    /// Required accessibility parent roles
    pub scope: Vec<String>,
}

/// The raw property and state values extracted from the WAI-ARIA spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScrapedAriaProperty {
    pub kind: PropertyKind,
    pub name: String,
    pub idl_name: Option<String>,
    pub description: Option<String>,
    pub is_global: bool,

    /// Used in roles
    pub applicability: Vec<String>,
    /// Inherits into roles
    pub descendants: Vec<String>,
    /// Related concepts
    pub related: Option<String>,
    pub value_kind: String,
    pub values: Vec<String>,
}

/// Whether a `ScrapedProperty` is an ARIA property or state
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PropertyKind {
    Property,
    State,
}

/// The raw element values extracted from the WAI-ARIA spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScrapedAriaElement {
    pub id: String,
    pub name: String,
    pub implicit_roles: Vec<String>,
    pub allowed_roles: Vec<String>,
    pub allowed_properties: Vec<String>,
    pub global: Option<String>,
    pub checked: Option<String>,
    pub strong: Vec<String>,
    pub links: Vec<String>,
}

/// Parse the W3C WAI-ARIA standards document.
pub fn scrape_aria(spec: String) -> Result<(Vec<ScrapedAriaRole>, Vec<ScrapedAriaProperty>)> {
    let document = scraper::Html::parse_document(&spec);
    let roles = scrape_aria_roles(&document)?;
    let properties = scrape_aria_properties_and_states(&document)?;
    Ok((roles, properties))
}

/// Parse the W3C ARIA in HTML standards document.
pub fn scrape_html_aria(spec: String) -> Result<Vec<ScrapedAriaElement>> {
    let document = scraper::Html::parse_document(&spec);
    let mut specs = vec![];

    let selector = scraper::Selector::parse("#docconformance").unwrap();
    let header = document.select(&selector).next().unwrap();
    let section = ElementRef::wrap(header.parent().unwrap()).unwrap();
    let selector = scraper::Selector::parse("table").unwrap();
    let table = section.select(&selector).next().unwrap();
    let selector = scraper::Selector::parse("tbody tr").unwrap();
    for row in table.select(&selector) {
        let id = extract_id("th", row).unwrap().to_owned();
        let element = extract_str("th", row).unwrap();
        let implicit_roles = extract_vec("td:nth-child(2) a[href^=\"#index-aria-\"]", row);

        let selector = scraper::Selector::parse("td:nth-child(3)").unwrap();
        let allowances = row.select(&selector).next().unwrap();
        let allowed_roles = extract_vec("a[href^=\"#index-aria-\"]", allowances);
        let allowed_properties = extract_vec("a[data-cite^=\"wai-aria-1.2#aria-\"]", allowances);
        let global = extract_str("a[data-cite=\"wai-aria-1.2#global_states\"]", allowances);
        let checked = extract_str("a[href=\"#att-checked\"]", allowances);
        let strong = extract_vec("strong", allowances);
        let links = extract_vec("a:not([href]):not([data-cite])", allowances);

        specs.push(ScrapedAriaElement {
            id,
            name: element,
            implicit_roles,
            allowed_roles,
            allowed_properties,
            global,
            checked,
            strong,
            links,
        })
    }
    Ok(specs)
}

/// Scrape the ARIA role definitions
fn scrape_aria_roles(document: &scraper::Html) -> Result<Vec<ScrapedAriaRole>> {
    let mut specs = vec![];

    let selector = scraper::Selector::parse(".role").unwrap();
    for element in document.select(&selector) {
        let Some(name) = extract_str(".role-name code", element) else {
            continue;
        };

        let implicit_values = extract_str(".implicit-values", element);
        let is_abstract = extract_bool(".role-abstract", element);
        let base = extract_str(".role-base", element);
        let are_children_presentational = extract_bool(".role-childpresentational", element);
        let children = extract_vec(".role-children code", element);
        let disallowed = extract_vec(".role-disallowed code", element);
        let inherited = extract_vec(".role-inherited code", element);
        let must_contain = extract_vec(".role-mustcontain code", element);
        let name_from = extract_str(".role-namefrom", element);
        let is_name_required = extract_bool(".role-namerequired", element);
        let parent = extract_vec(".role-parent code", element);
        let properties = extract_vec(".role-properties code", element);
        let related = extract_str(".role-related", element);
        let required = extract_vec(".required-properties code", element);
        let scope = extract_vec(".role-scope code", element);

        specs.push(ScrapedAriaRole {
            name,
            implicit_values,
            is_abstract,
            base,
            are_children_presentational,
            children,
            disallowed,
            inherited,
            must_contain,
            name_from,
            is_name_required,
            parent,
            properties,
            related,
            required,
            scope,
        })
    }

    Ok(specs)
}

/// Scrape the ARIA property and state definitions
fn scrape_aria_properties_and_states(document: &scraper::Html) -> Result<Vec<ScrapedAriaProperty>> {
    let mut global_properties = vec![];
    let selector = scraper::Selector::parse("#global_states li a").unwrap();
    for element in document.select(&selector) {
        global_properties.push(element.value().attr("href").unwrap()[1..].to_string());
    }

    let mut descriptions = HashMap::new();
    let dt_selector = scraper::Selector::parse("dl#index_state_prop dt").unwrap();
    let dd_selector = scraper::Selector::parse("dl#index_state_prop dd").unwrap();
    for (dt, dd) in document
        .select(&dt_selector)
        .zip(document.select(&dd_selector))
    {
        descriptions.insert(dt.text().collect::<String>(), dd.text().collect::<String>());
    }

    let mut idl_attribute_names = HashMap::new();
    let selector =
        scraper::Selector::parse("#accessibilityroleandproperties-correspondence tr").unwrap();
    for row in document.select(&selector) {
        if let Some(idl) = extract_str("[data-idl=\"attribute\"]", row) {
            if let Some(property) = extract_str(".property-reference, .state-reference", row) {
                idl_attribute_names.insert(property, idl);
            }
        }
    }

    let mut specs = vec![];

    let selector = scraper::Selector::parse(".property, .state").unwrap();
    for element in document.select(&selector) {
        let Some(name) = extract_str(".property-name code, .state-name code", element) else {
            continue;
        };
        let idl_name = idl_attribute_names.get(&name).cloned();
        let description = descriptions.remove(&name);

        let kind = if element.value().classes().any(|x| x == "property") {
            PropertyKind::Property
        } else {
            PropertyKind::State
        };

        let is_global = global_properties.contains(&name);
        let applicability = extract_vec(
            ".property-applicability code, .state-applicability code",
            element,
        );
        let descendants = extract_vec(
            ".property-descendants code, .state-descendants code",
            element,
        );
        let related = extract_str(".property-related, .state-related", element);
        let value_kind = extract_str(".property-value, .state-value", element).unwrap();
        let values = extract_vec(".value-name", element);

        specs.push(ScrapedAriaProperty {
            kind,
            name,
            idl_name,
            description,
            is_global,
            applicability,
            descendants,
            related,
            value_kind,
            values,
        });
    }

    Ok(specs)
}

/// Attempt to extract the id attribute of `selector` from `element`.
fn extract_id<'a>(selector: &str, element: scraper::ElementRef<'a>) -> Option<&'a str> {
    let selector = scraper::Selector::parse(selector).unwrap();
    element
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("id"))
}

/// Attempt to extract the text content of `selector` from `element`.
fn extract_str(selector: &str, element: scraper::ElementRef) -> Option<String> {
    let selector = scraper::Selector::parse(selector).unwrap();
    element
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_owned())
}

/// Extract a boolean value from `element` using `selector`.
///
/// If the selector matches an element and that element's text content is "True" then
/// return `true`, else return `false`.
fn extract_bool(selector: &str, element: scraper::ElementRef) -> bool {
    let selector = scraper::Selector::parse(selector).unwrap();
    if let Some(el) = element.select(&selector).next() {
        if el.text().next() == Some("True") {
            return true;
        }
    }

    false
}

/// Extract a list of `String`s from `element` using `selector`
fn extract_vec(selector: &str, element: scraper::ElementRef) -> Vec<String> {
    let selector = scraper::Selector::parse(selector).unwrap();
    element
        .select(&selector)
        .map(|el| el.text().collect())
        .collect()
}
