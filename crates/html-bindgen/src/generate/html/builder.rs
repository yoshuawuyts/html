use crate::parse::{Attribute, AttributeType};
use convert_case::{Case, Casing};

pub(crate) fn gen_builder(
    struct_name: &str,
    permitted_child_elements: &[String],
    method_attributes: &[Attribute],
) -> String {
    let builder_ty = format!("{struct_name}Builder");
    let struct_ty = format!("super::element::{struct_name}");
    let method_names = permitted_child_elements
        .iter()
        .map(|element_ty| element_ty.to_case(Case::Snake))
        .collect::<Vec<_>>();

    let has_children = !permitted_child_elements.is_empty();

    let element_methods = gen_element_methods(permitted_child_elements);
    let push_append_methods = gen_push_append_methods(&struct_name, has_children);
    let attr_methods = gen_attr_methods(&method_names, method_attributes);

    format!(
        "
        /// A builder struct for {struct_name}
        pub struct {builder_ty} {{
            element: {struct_ty},
        }}
    
        impl {builder_ty} {{
            pub(crate) fn new(element: {struct_ty}) -> Self {{
                Self {{ element }}
            }}

            /// Finish building the element
            pub fn build(&mut self) -> {struct_ty} {{
                self.element.clone()
            }}

            /// Insert a `data-*` property
            pub fn data(&mut self, data_key: impl Into<std::borrow::Cow<'static, str>>, value: impl Into<std::borrow::Cow<'static, str>>) -> &mut {builder_ty} {{
                self.element.data_map_mut().insert(data_key.into(), value.into());
                self
            }}
    
            {element_methods}
            {attr_methods}
            {push_append_methods}
        }}
        "
    )
}

fn gen_push_append_methods(struct_name: &str, has_children: bool) -> String {
    if !has_children {
        return String::new();
    }
    let child_ty = format!("crate::generated::all::children::{struct_name}Child");
    format!(
        "
        /// Push a new child element to the list of children.
        pub fn push<T>(&mut self, child_el: T) -> &mut Self
        where
            T: Into<{child_ty}>,
        {{
            let child_el = child_el.into();
            self.element.children_mut().push(child_el);
            self
        }}

        /// Extend the list of children with an iterator of child elements.
        pub fn extend<I, T>(&mut self, iter: I) -> &mut Self 
        where
            I: IntoIterator<Item = T>,
            T: Into<{child_ty}>,
        {{
            let iter = iter.into_iter()
                .map(|child_el| child_el.into());
            self.element.children_mut().extend(iter);
            self
        }}
    "
    )
}

fn gen_element_methods(permitted_child_elements: &[String]) -> String {
    permitted_child_elements
        .iter()
        .map(|element_ty| {
            let method_name = match element_ty.to_case(Case::Snake).as_str() {
                "data" => "data_el".to_owned(),
                other => other.to_owned(),
            };

            match element_ty.as_str() {
                "Text" => {
                    // String::new()
                    format!("/// Append a new text element.
                            pub fn text(&mut self, s: impl Into<std::borrow::Cow<'static, str>>) -> &mut Self {{
                                let cow = s.into();
                                self.element.children_mut().push(cow.into());
                                self
                            }}"
                        )
                }
                element_ty => {
                    let ty = format!("crate::generated::all::{element_ty}");
                    let ty_builder =
                        format!("crate::generated::all::builders::{element_ty}Builder");
                    format!(
                        "/// Append a new `{element_ty}` element
                        pub fn {method_name}<F>(&mut self, f: F) -> &mut Self
                        where F:
                            for<'a> FnOnce(&'a mut {ty_builder}) -> &'a mut {ty_builder}
                        {{
                            let ty: {ty} = Default::default();
                            let mut ty_builder = {ty_builder}::new(ty);
                            (f)(&mut ty_builder);
                            let ty = ty_builder.build();
                            self.element.children_mut().push(ty.into());
                            self
                        }}"
                    )
                }
            }
        })
        .collect()
}

fn gen_attr_methods(permitted_child_elements: &[String], attributes: &[Attribute]) -> String {
    attributes
        .into_iter()
        .map(|attr| {
            let name = &attr.name;
            let field_name = &attr.field_name;

            // A field name may conflict with element name. In which case we
            // suffix the method name to include `_attr`.
            let method_name = if permitted_child_elements.contains(field_name) {
                format!("{field_name}_attr")
            } else {
                field_name.clone()
            };

            let method_name = match method_name.as_str() {
                "data" => "data_attr".to_owned(),
                other => other.to_owned(),
            };

            let param_ty = match &attr.ty {
                AttributeType::Bool => "bool".to_owned(),
                AttributeType::String => "impl Into<std::borrow::Cow<'static, str>>".to_owned(),
                ty => format!("{ty}"),
            };

            let field_setter = match &attr.ty {
                AttributeType::String => format!("Some(value.into())"),
                AttributeType::Bool => format!("value"),
                _ => format!("Some(value)"),
            };
            format!(
                "
            /// Set the value of the `{name}` attribute
            pub fn {method_name}(&mut self, value: {param_ty}) -> &mut Self {{
                self.element.set_{field_name}({field_setter});
                self
            }}",
            )
        })
        .collect()
}
