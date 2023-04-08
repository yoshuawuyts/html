/// The HTML `<sup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
#[doc(alias = "sup")]
#[non_exhaustive]
pub struct SuperScript {
    _sys: html_sys::text::SuperScript,
}
impl crate::categories::FlowContent for SuperScript {}
impl crate::categories::PhrasingContent for SuperScript {}
impl crate::categories::PalpableContent for SuperScript {}
