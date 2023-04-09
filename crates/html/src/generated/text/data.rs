/// The HTML `<data>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
#[doc(alias = "data")]
#[non_exhaustive]
pub struct Data {
    sys: html_sys::text::Data,
    _children: Vec<()>,
}
impl Data {
    /// Get the value of the `value` attribute
    pub fn value(&self) -> std::option::Option<&str> {
        self.sys.value.as_deref()
    }
    /// Set the value of the `value` attribute
    pub fn set_value(&mut self, value: std::option::Option<String>) {
        self.sys.value = value;
    }
}
impl crate::HtmlElement for Data {}
impl crate::categories::FlowContent for Data {}
impl crate::categories::PhrasingContent for Data {}
impl crate::categories::PalpableContent for Data {}
impl std::convert::Into<html_sys::text::Data> for Data {
    fn into(self) -> html_sys::text::Data {
        self.sys
    }
}
impl From<html_sys::text::Data> for Data {
    fn from(sys: html_sys::text::Data) -> Self {
        Self { sys, _children: vec![] }
    }
}
