/// The HTML `<embed>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
#[doc(alias = "embed")]
#[non_exhaustive]
pub struct Embed {
    _sys: html_sys::embedded::Embed,
}
impl crate::categories::FlowContent for Embed {}
impl crate::categories::PhrasingContent for Embed {}
impl crate::categories::EmbeddedContent for Embed {}
impl crate::categories::InteractiveContent for Embed {}
impl crate::categories::PalpableContent for Embed {}
