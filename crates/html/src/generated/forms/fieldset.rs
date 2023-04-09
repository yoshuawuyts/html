/// The HTML `<fieldset>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
#[doc(alias = "fieldset")]
#[non_exhaustive]
pub struct Fieldset {
    sys: html_sys::forms::Fieldset,
}
impl Fieldset {
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
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
}
impl crate::categories::FlowContent for Fieldset {}
impl crate::categories::PalpableContent for Fieldset {}
impl std::convert::Into<html_sys::forms::Fieldset> for Fieldset {
    fn into(self) -> html_sys::forms::Fieldset {
        self.sys
    }
}
impl From<html_sys::forms::Fieldset> for Fieldset {
    fn from(sys: html_sys::forms::Fieldset) -> Self {
        Self { sys }
    }
}
