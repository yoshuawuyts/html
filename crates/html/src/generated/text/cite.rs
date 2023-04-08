/// The HTML `<cite>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
#[doc(alias = "cite")]
#[non_exhaustive]
pub struct Cite {
    _sys: html_sys::text::Cite,
}
impl crate::categories::FlowContent for Cite {}
impl crate::categories::PhrasingContent for Cite {}
impl crate::categories::PalpableContent for Cite {}
