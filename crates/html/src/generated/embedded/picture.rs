/// The HTML `<picture>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture)
#[doc(alias = "picture")]
#[non_exhaustive]
pub struct Picture {
    _sys: html_sys::embedded::Picture,
}
impl crate::categories::FlowContent for Picture {}
impl crate::categories::PhrasingContent for Picture {}
impl crate::categories::EmbeddedContent for Picture {}
impl crate::categories::PalpableContent for Picture {}
