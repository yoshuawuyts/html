/// The HTML `<cite>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
#[doc(alias = "cite")]
#[non_exhaustive]
pub struct Cite {
    sys: html_sys::text::Cite,
    _children: Vec<()>,
}
impl crate::HtmlElement for Cite {}
impl crate::categories::FlowContent for Cite {}
impl crate::categories::PhrasingContent for Cite {}
impl crate::categories::PalpableContent for Cite {}
impl std::convert::Into<html_sys::text::Cite> for Cite {
    fn into(self) -> html_sys::text::Cite {
        self.sys
    }
}
impl From<html_sys::text::Cite> for Cite {
    fn from(sys: html_sys::text::Cite) -> Self {
        Self { sys, _children: vec![] }
    }
}
