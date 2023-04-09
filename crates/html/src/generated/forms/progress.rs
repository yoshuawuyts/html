/// The HTML `<progress>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
#[doc(alias = "progress")]
#[non_exhaustive]
pub struct Progress {
    sys: html_sys::forms::Progress,
}
impl Progress {
    /// Get the value of the `value` attribute
    pub fn value(&self) -> std::option::Option<&str> {
        self.sys.value.as_deref()
    }
    /// Set the value of the `value` attribute
    pub fn set_value(&mut self, value: std::option::Option<String>) {
        self.sys.value = value;
    }
    /// Get the value of the `max` attribute
    pub fn max(&self) -> std::option::Option<&str> {
        self.sys.max.as_deref()
    }
    /// Set the value of the `max` attribute
    pub fn set_max(&mut self, value: std::option::Option<String>) {
        self.sys.max = value;
    }
}
impl crate::categories::FlowContent for Progress {}
impl crate::categories::PhrasingContent for Progress {}
impl crate::categories::PalpableContent for Progress {}
impl std::convert::Into<html_sys::forms::Progress> for Progress {
    fn into(self) -> html_sys::forms::Progress {
        self.sys
    }
}
impl From<html_sys::forms::Progress> for Progress {
    fn from(sys: html_sys::forms::Progress) -> Self {
        Self { sys }
    }
}
