/// The HTML `<span>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
#[doc(alias = "span")]
#[non_exhaustive]
pub struct Span {
    sys: html_sys::text::Span,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Span {}
impl crate::categories::PhrasingContent for Span {}
impl crate::categories::PalpableContent for Span {}
impl std::convert::Into<html_sys::text::Span> for Span {
    fn into(self) -> html_sys::text::Span {
        self.sys
    }
}
impl From<html_sys::text::Span> for Span {
    fn from(sys: html_sys::text::Span) -> Self {
        Self { sys, _children: vec![] }
    }
}
