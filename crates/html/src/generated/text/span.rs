/// The HTML `<span>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
#[doc(alias = "span")]
#[non_exhaustive]
pub struct Span {
    _sys: html_sys::text::Span,
}
impl crate::categories::FlowContent for Span {}
impl crate::categories::PhrasingContent for Span {}
impl crate::categories::PalpableContent for Span {}
