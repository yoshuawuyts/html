use super::types::*;
use convert_case::{Case, Casing};
use indoc::formatdoc;

pub(crate) fn def_to_string(def: Definition) -> String {
    let Definition {
        tag_name,
        dom_interface,
        inherits_from,
        members,
    } = def;

    let name = normalize_ident(&tag_name);

    let is_element = name.starts_with("HTML") && name.ends_with("Element") && name != "HTMLElement";
    let struct_ident = generic_name(&name);
    let impl_ident = "impl";

    let (field, inherits) = match inherits_from {
        Some(from) => {
            let from = generic_name(&from);
            let inherits = formatdoc!(
                "
                {impl_ident} ::std::ops::Deref for {struct_ident} {{
                    type Target = {from};
                    fn deref(&self) -> &Self::Target {{
                        &self.deref_target
                    }}
                }}

                {impl_ident} ::std::ops::DerefMut for {struct_ident} {{
                    fn deref_mut(&mut self) -> &mut Self::Target {{
                        &mut self.deref_target
                    }}
                }}
                "
            );
            let field = formatdoc!("deref_target: {from},");
            (field, inherits)
        }
        None => (String::new(), String::new()),
    };

    let mut fields = vec![field];

    if is_element {
        fields.push("children: usize,".to_owned());
    }
    let fields_iter = members.iter().filter_map(|member| match member.read_only {
        true => None,
        false => Some(format!(
            "    {}: {},",
            normalize_ident(&member.name.to_case(Case::Snake)),
            generic_name(&member.ty.to_string()),
        )),
    });
    fields.extend(fields_iter);
    let fields = fields.join("\n");

    let mut methods = vec![];
    let methods_iter = members.iter().filter_map(|member| match member.read_only {
        true => None,
        false => Some(formatdoc!(
            "pub fn {name}(&self) -> {ty} {{
                {base_ty}::clone(&self.{name})
            }}

            pub fn set_{name}(&mut self, value: {ty}) {{
                self.{name} = value;
            }}",
            name = normalize_ident(&member.name.to_case(Case::Snake)),
            base_ty = &member.ty,
            ty = generic_name(&member.ty.to_string()),
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
        let tag_name = tag_name;
        let html_impl = formatdoc!("{impl_ident} HtmlElement for {struct_ident} {{}}\n");
        let display_impl = formatdoc!(
            "{impl_ident} ::std::fmt::Display for {struct_ident} {{
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
                    write!(f, \"<{tag_name}>\")?;
                    write!(f, \"<{{}}>\", self.children)?;
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

fn generic_name(name: &str) -> String {
    format!("{name}")
}
