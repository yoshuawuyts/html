/// The HTML `<code>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
#[doc(alias = "code")]
#[non_exhaustive]
pub struct Code {
    sys: html_sys::text::Code,
    _children: Vec<()>,
}
impl crate::HtmlElement for Code {}
impl crate::categories::FlowContent for Code {}
impl crate::categories::PhrasingContent for Code {}
impl crate::categories::PalpableContent for Code {}
impl std::convert::Into<html_sys::text::Code> for Code {
    fn into(self) -> html_sys::text::Code {
        self.sys
    }
}
impl From<html_sys::text::Code> for Code {
    fn from(sys: html_sys::text::Code) -> Self {
        Self { sys, _children: vec![] }
    }
}
