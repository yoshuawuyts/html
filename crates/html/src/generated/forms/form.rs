/// The HTML `<form>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
#[doc(alias = "form")]
#[non_exhaustive]
pub struct Form {
    sys: html_sys::forms::Form,
    _children: Vec<()>,
}
impl Form {
    /// Get the value of the `accept-charset` attribute
    pub fn accept_charset(&self) -> std::option::Option<&str> {
        self.sys.accept_charset.as_deref()
    }
    /// Set the value of the `accept-charset` attribute
    pub fn set_accept_charset(&mut self, value: std::option::Option<String>) {
        self.sys.accept_charset = value;
    }
    /// Get the value of the `action` attribute
    pub fn action(&self) -> std::option::Option<&str> {
        self.sys.action.as_deref()
    }
    /// Set the value of the `action` attribute
    pub fn set_action(&mut self, value: std::option::Option<String>) {
        self.sys.action = value;
    }
    /// Get the value of the `autocomplete` attribute
    pub fn autocomplete(&self) -> std::option::Option<&str> {
        self.sys.autocomplete.as_deref()
    }
    /// Set the value of the `autocomplete` attribute
    pub fn set_autocomplete(&mut self, value: std::option::Option<String>) {
        self.sys.autocomplete = value;
    }
    /// Get the value of the `enctype` attribute
    pub fn enctype(&self) -> std::option::Option<&str> {
        self.sys.enctype.as_deref()
    }
    /// Set the value of the `enctype` attribute
    pub fn set_enctype(&mut self, value: std::option::Option<String>) {
        self.sys.enctype = value;
    }
    /// Get the value of the `method` attribute
    pub fn method(&self) -> std::option::Option<&str> {
        self.sys.method.as_deref()
    }
    /// Set the value of the `method` attribute
    pub fn set_method(&mut self, value: std::option::Option<String>) {
        self.sys.method = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `novalidate` attribute
    pub fn no_validate(&self) -> bool {
        self.sys.no_validate
    }
    /// Set the value of the `novalidate` attribute
    pub fn set_no_validate(&mut self, value: bool) {
        self.sys.no_validate = value;
    }
    /// Get the value of the `target` attribute
    pub fn target(&self) -> std::option::Option<&str> {
        self.sys.target.as_deref()
    }
    /// Set the value of the `target` attribute
    pub fn set_target(&mut self, value: std::option::Option<String>) {
        self.sys.target = value;
    }
}
impl crate::categories::FlowContent for Form {}
impl crate::categories::PalpableContent for Form {}
impl std::convert::Into<html_sys::forms::Form> for Form {
    fn into(self) -> html_sys::forms::Form {
        self.sys
    }
}
impl From<html_sys::forms::Form> for Form {
    fn from(sys: html_sys::forms::Form) -> Self {
        Self { sys, _children: vec![] }
    }
}
