/// The HTML `<iframe>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
#[doc(alias = "iframe")]
#[non_exhaustive]
pub struct Iframe {
    _sys: html_sys::embedded::Iframe,
}
impl crate::categories::FlowContent for Iframe {}
impl crate::categories::PhrasingContent for Iframe {}
impl crate::categories::EmbeddedContent for Iframe {}
impl crate::categories::InteractiveContent for Iframe {}
impl crate::categories::PalpableContent for Iframe {}
