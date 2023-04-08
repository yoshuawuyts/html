/// The HTML `<u>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
#[doc(alias = "u")]
#[non_exhaustive]
pub struct Underline {
    _sys: html_sys::text::Underline,
}
impl crate::categories::FlowContent for Underline {}
impl crate::categories::PhrasingContent for Underline {}
impl crate::categories::PalpableContent for Underline {}
