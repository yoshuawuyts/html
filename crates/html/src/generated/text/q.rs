/// The HTML `<q>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q)
#[doc(alias = "q")]
#[non_exhaustive]
pub struct Quotation {
    sys: html_sys::text::Quotation,
    _children: Vec<()>,
}
impl Quotation {
    /// Get the value of the `cite` attribute
    pub fn cite(&self) -> std::option::Option<&str> {
        self.sys.cite.as_deref()
    }
    /// Set the value of the `cite` attribute
    pub fn set_cite(&mut self, value: std::option::Option<String>) {
        self.sys.cite = value;
    }
}
impl crate::HtmlElement for Quotation {}
impl crate::categories::FlowContent for Quotation {}
impl crate::categories::PhrasingContent for Quotation {}
impl crate::categories::PalpableContent for Quotation {}
impl std::convert::Into<html_sys::text::Quotation> for Quotation {
    fn into(self) -> html_sys::text::Quotation {
        self.sys
    }
}
impl From<html_sys::text::Quotation> for Quotation {
    fn from(sys: html_sys::text::Quotation) -> Self {
        Self { sys, _children: vec![] }
    }
}
