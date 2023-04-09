/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
pub struct Button {
    sys: html_sys::forms::Button,
    _children: Vec<()>,
}
impl Button {
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
    /// Get the value of the `formaction` attribute
    pub fn form_action(&self) -> std::option::Option<&str> {
        self.sys.form_action.as_deref()
    }
    /// Set the value of the `formaction` attribute
    pub fn set_form_action(&mut self, value: std::option::Option<String>) {
        self.sys.form_action = value;
    }
    /// Get the value of the `formenctype` attribute
    pub fn form_enctype(&self) -> std::option::Option<&str> {
        self.sys.form_enctype.as_deref()
    }
    /// Set the value of the `formenctype` attribute
    pub fn set_form_enctype(&mut self, value: std::option::Option<String>) {
        self.sys.form_enctype = value;
    }
    /// Get the value of the `formmethod` attribute
    pub fn form_method(&self) -> std::option::Option<&str> {
        self.sys.form_method.as_deref()
    }
    /// Set the value of the `formmethod` attribute
    pub fn set_form_method(&mut self, value: std::option::Option<String>) {
        self.sys.form_method = value;
    }
    /// Get the value of the `formnovalidate` attribute
    pub fn form_no_validate(&self) -> bool {
        self.sys.form_no_validate
    }
    /// Set the value of the `formnovalidate` attribute
    pub fn set_form_no_validate(&mut self, value: bool) {
        self.sys.form_no_validate = value;
    }
    /// Get the value of the `formtarget` attribute
    pub fn form_target(&self) -> std::option::Option<&str> {
        self.sys.form_target.as_deref()
    }
    /// Set the value of the `formtarget` attribute
    pub fn set_form_target(&mut self, value: std::option::Option<String>) {
        self.sys.form_target = value;
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
        Self { sys, _children: vec![] }
    }
}
