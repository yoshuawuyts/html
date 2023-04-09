/// The HTML `<output>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
#[doc(alias = "output")]
#[non_exhaustive]
pub struct Output {
    sys: html_sys::forms::Output,
    _children: Vec<T>,
}
impl Output {
    /// Get the value of the `for` attribute
    pub fn for_(&self) -> std::option::Option<&str> {
        self.sys.for_.as_deref()
    }
    /// Set the value of the `for` attribute
    pub fn set_for_(&mut self, value: std::option::Option<String>) {
        self.sys.for_ = value;
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
impl crate::categories::FlowContent for Output {}
impl crate::categories::PhrasingContent for Output {}
impl crate::categories::PalpableContent for Output {}
impl std::convert::Into<html_sys::forms::Output> for Output {
    fn into(self) -> html_sys::forms::Output {
        self.sys
    }
}
impl From<html_sys::forms::Output> for Output {
    fn from(sys: html_sys::forms::Output) -> Self {
        Self { sys, _children: vec![] }
    }
}
