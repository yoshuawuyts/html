/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
pub struct Button {
    sys: html_sys::forms::Button,
}
impl Button {
    /// Get the value of the `disabled` attribute
    pub fn disabled(&self) -> std::option::Option<&str> {
        self.sys.disabled.as_deref()
    }
    /// Set the value of the `disabled` attribute
    pub fn set_disabled(&mut self, value: std::option::Option<String>) {
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
    /// Get the value of the `formaction` attribute
    pub fn formaction(&self) -> std::option::Option<&str> {
        self.sys.formaction.as_deref()
    }
    /// Set the value of the `formaction` attribute
    pub fn set_formaction(&mut self, value: std::option::Option<String>) {
        self.sys.formaction = value;
    }
    /// Get the value of the `formenctype` attribute
    pub fn formenctype(&self) -> std::option::Option<&str> {
        self.sys.formenctype.as_deref()
    }
    /// Set the value of the `formenctype` attribute
    pub fn set_formenctype(&mut self, value: std::option::Option<String>) {
        self.sys.formenctype = value;
    }
    /// Get the value of the `formmethod` attribute
    pub fn formmethod(&self) -> std::option::Option<&str> {
        self.sys.formmethod.as_deref()
    }
    /// Set the value of the `formmethod` attribute
    pub fn set_formmethod(&mut self, value: std::option::Option<String>) {
        self.sys.formmethod = value;
    }
    /// Get the value of the `formnovalidate` attribute
    pub fn formnovalidate(&self) -> std::option::Option<&str> {
        self.sys.formnovalidate.as_deref()
    }
    /// Set the value of the `formnovalidate` attribute
    pub fn set_formnovalidate(&mut self, value: std::option::Option<String>) {
        self.sys.formnovalidate = value;
    }
    /// Get the value of the `formtarget` attribute
    pub fn formtarget(&self) -> std::option::Option<&str> {
        self.sys.formtarget.as_deref()
    }
    /// Set the value of the `formtarget` attribute
    pub fn set_formtarget(&mut self, value: std::option::Option<String>) {
        self.sys.formtarget = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
    /// Get the value of the `value` attribute
    pub fn value(&self) -> std::option::Option<&str> {
        self.sys.value.as_deref()
    }
    /// Set the value of the `value` attribute
    pub fn set_value(&mut self, value: std::option::Option<String>) {
        self.sys.value = value;
    }
}
impl crate::categories::FlowContent for Button {}
impl crate::categories::PhrasingContent for Button {}
impl crate::categories::InteractiveContent for Button {}
impl crate::categories::PalpableContent for Button {}
impl std::convert::Into<html_sys::forms::Button> for Button {
    fn into(self) -> html_sys::forms::Button {
        self.sys
    }
}
impl From<html_sys::forms::Button> for Button {
    fn from(sys: html_sys::forms::Button) -> Self {
        Self { sys }
    }
}
