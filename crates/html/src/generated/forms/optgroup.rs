/// The HTML `<optgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
#[doc(alias = "optgroup")]
#[non_exhaustive]
pub struct OptionGroup {
    sys: html_sys::forms::OptionGroup,
}
impl OptionGroup {
    /// Get the value of the `disabled` attribute
    pub fn disabled(&self) -> std::option::Option<&str> {
        self.sys.disabled.as_deref()
    }
    /// Set the value of the `disabled` attribute
    pub fn set_disabled(&mut self, value: std::option::Option<String>) {
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
}
impl std::convert::Into<html_sys::forms::OptionGroup> for OptionGroup {
    fn into(self) -> html_sys::forms::OptionGroup {
        self.sys
    }
}
impl From<html_sys::forms::OptionGroup> for OptionGroup {
    fn from(sys: html_sys::forms::OptionGroup) -> Self {
        Self { sys }
    }
}
