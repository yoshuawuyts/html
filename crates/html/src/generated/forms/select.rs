/// The HTML `<select>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
#[doc(alias = "select")]
#[non_exhaustive]
pub struct Select {
    sys: html_sys::forms::Select,
    _children: Vec<()>,
}
impl Select {
    /// Get the value of the `autocomplete` attribute
    pub fn autocomplete(&self) -> std::option::Option<&str> {
        self.sys.autocomplete.as_deref()
    }
    /// Set the value of the `autocomplete` attribute
    pub fn set_autocomplete(&mut self, value: std::option::Option<String>) {
        self.sys.autocomplete = value;
    }
    /// Get the value of the `disabled` attribute
    pub fn disabled(&self) -> bool {
        self.sys.disabled
    }
    /// Set the value of the `disabled` attribute
    pub fn set_disabled(&mut self, value: bool) {
        self.sys.disabled = value;
    }
    /// Get the value of the `form` attribute
    pub fn form(&self) -> std::option::Option<&str> {
        self.sys.form.as_deref()
    }
    /// Set the value of the `form` attribute
    pub fn set_form(&mut self, value: std::option::Option<String>) {
        self.sys.form = value;
    }
    /// Get the value of the `multiple` attribute
    pub fn multiple(&self) -> bool {
        self.sys.multiple
    }
    /// Set the value of the `multiple` attribute
    pub fn set_multiple(&mut self, value: bool) {
        self.sys.multiple = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `required` attribute
    pub fn required(&self) -> bool {
        self.sys.required
    }
    /// Set the value of the `required` attribute
    pub fn set_required(&mut self, value: bool) {
        self.sys.required = value;
    }
    /// Get the value of the `size` attribute
    pub fn size(&self) -> std::option::Option<i64> {
        self.sys.size
    }
    /// Set the value of the `size` attribute
    pub fn set_size(&mut self, value: std::option::Option<i64>) {
        self.sys.size = value;
    }
}
impl crate::HtmlElement for Select {}
impl crate::categories::FlowContent for Select {}
impl crate::categories::PhrasingContent for Select {}
impl crate::categories::InteractiveContent for Select {}
impl crate::categories::PalpableContent for Select {}
impl std::convert::Into<html_sys::forms::Select> for Select {
    fn into(self) -> html_sys::forms::Select {
        self.sys
    }
}
impl From<html_sys::forms::Select> for Select {
    fn from(sys: html_sys::forms::Select) -> Self {
        Self { sys, _children: vec![] }
    }
}
