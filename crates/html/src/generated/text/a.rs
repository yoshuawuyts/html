/// The HTML `<a>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
#[doc(alias = "a")]
#[non_exhaustive]
pub struct Anchor {
    _sys: html_sys::text::Anchor,
}
impl crate::categories::FlowContent for Anchor {}
impl crate::categories::PhrasingContent for Anchor {}
impl crate::categories::PalpableContent for Anchor {}
