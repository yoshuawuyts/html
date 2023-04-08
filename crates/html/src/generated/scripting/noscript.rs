/// The HTML `<noscript>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript)
#[doc(alias = "noscript")]
#[non_exhaustive]
pub struct NoScript {
    _sys: html_sys::scripting::NoScript,
}
impl crate::categories::MetadataContent for NoScript {}
impl crate::categories::FlowContent for NoScript {}
impl crate::categories::PhrasingContent for NoScript {}
