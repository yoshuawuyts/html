/// The HTML `<sub>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
#[doc(alias = "sub")]
#[non_exhaustive]
pub struct SubScript {
    _sys: html_sys::text::SubScript,
}
impl crate::categories::FlowContent for SubScript {}
impl crate::categories::PhrasingContent for SubScript {}
impl crate::categories::PalpableContent for SubScript {}
