/// The HTML `<object>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object)
#[doc(alias = "object")]
#[non_exhaustive]
pub struct Object {
    sys: html_sys::embedded::Object,
    _children: Vec<()>,
}
impl Object {
    /// Get the value of the `data` attribute
    pub fn data(&self) -> std::option::Option<&str> {
        self.sys.data.as_deref()
    }
    /// Set the value of the `data` attribute
    pub fn set_data(&mut self, value: std::option::Option<String>) {
        self.sys.data = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `form` attribute
    pub fn form(&self) -> std::option::Option<&str> {
        self.sys.form.as_deref()
    }
    /// Set the value of the `form` attribute
    pub fn set_form(&mut self, value: std::option::Option<String>) {
        self.sys.form = value;
    }
    /// Get the value of the `width` attribute
    pub fn width(&self) -> std::option::Option<&str> {
        self.sys.width.as_deref()
    }
    /// Set the value of the `width` attribute
    pub fn set_width(&mut self, value: std::option::Option<String>) {
        self.sys.width = value;
    }
    /// Get the value of the `height` attribute
    pub fn height(&self) -> std::option::Option<&str> {
        self.sys.height.as_deref()
    }
    /// Set the value of the `height` attribute
    pub fn set_height(&mut self, value: std::option::Option<String>) {
        self.sys.height = value;
    }
}
impl crate::HtmlElement for Object {}
impl crate::categories::FlowContent for Object {}
impl crate::categories::PhrasingContent for Object {}
impl crate::categories::EmbeddedContent for Object {}
impl crate::categories::PalpableContent for Object {}
impl std::convert::Into<html_sys::embedded::Object> for Object {
    fn into(self) -> html_sys::embedded::Object {
        self.sys
    }
}
impl From<html_sys::embedded::Object> for Object {
    fn from(sys: html_sys::embedded::Object) -> Self {
        Self { sys, _children: vec![] }
    }
}
