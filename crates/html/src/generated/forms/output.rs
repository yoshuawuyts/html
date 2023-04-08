/// The HTML `<output>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
#[doc(alias = "output")]
#[non_exhaustive]
pub struct Output {
    _sys: html_sys::forms::Output,
}
impl crate::categories::FlowContent for Output {}
impl crate::categories::PhrasingContent for Output {}
impl crate::categories::PalpableContent for Output {}
