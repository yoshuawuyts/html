/// The HTML `<option>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option)
#[doc(alias = "option")]
#[non_exhaustive]
pub struct Option {
    sys: html_sys::forms::Option,
}
impl Option {
    /// Get the value of the `disabled` attribute
    pub fn disabled(&self) -> bool {
        self.sys.disabled
    }
    /// Set the value of the `disabled` attribute
    pub fn set_disabled(&mut self, value: bool) {
        self.sys.disabled = value;
    }
    /// Get the value of the `label` attribute
    pub fn label(&self) -> std::option::Option<&str> {
        self.sys.label.as_deref()
    }
    /// Set the value of the `label` attribute
    pub fn set_label(&mut self, value: std::option::Option<String>) {
        self.sys.label = value;
    }
    /// Get the value of the `selected` attribute
    pub fn selected(&self) -> bool {
        self.sys.selected
    }
    /// Set the value of the `selected` attribute
    pub fn set_selected(&mut self, value: bool) {
        self.sys.selected = value;
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
impl crate::HtmlElement for Option {}
impl std::convert::Into<html_sys::forms::Option> for Option {
    fn into(self) -> html_sys::forms::Option {
        self.sys
    }
}
impl From<html_sys::forms::Option> for Option {
    fn from(sys: html_sys::forms::Option) -> Self {
        Self { sys }
    }
}
