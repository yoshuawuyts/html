use crate::parse::Attribute;
use convert_case::{Case, Casing};

pub(crate) fn gen_builder(
    struct_name: &str,
    permitted_child_elements: &[String],
    method_attributes: &[Attribute],
) -> String {
    let builder_name = format!("{struct_name}Builder");
    let struct_ty = format!("super::element::{struct_name}");
    let element_methods = gen_element_methods(permitted_child_elements);

    format!(
        "
        /// A builder struct for {struct_name}
        pub struct {builder_name} {{
            element: {struct_ty},
        }}
    
        impl {builder_name} {{
            pub(crate) fn new(element: {struct_ty}) -> Self {{
                Self {{ element }}
            }}

            /// Finish building the element
            pub fn build(&mut self) -> {struct_ty} {{
                self.element.clone()
            }}
    
            {element_methods}
        }}
        "
    )
}

fn gen_element_methods(permitted_child_elements: &[String]) -> String {
    permitted_child_elements
        .iter()
        .map(|element_ty| {
            let method_name = element_ty.to_case(Case::Snake);
            let ty = match element_ty.as_str() {
                "Text" => format!("std::borrow::Cow<'static, str>"),
                element_ty => format!("crate::generated::all::{element_ty}"),
            };
            format!(
                "
            /// Append a new `{element_ty}` element
            pub fn {method_name}<F>(&mut self, f: F) -> &mut Self
            where F:
                FnOnce(&mut {ty})
            {{
                let mut ty = Default::default();
                (f)(&mut ty);
                self.element.children_mut().push(ty.into());
                self
            }}
        "
            )
        })
        .collect()
}
