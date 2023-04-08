/// The HTML `<progress>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
#[doc(alias = "progress")]
#[non_exhaustive]
pub struct Progress {
    _sys: html_sys::forms::Progress,
}
impl crate::categories::FlowContent for Progress {}
impl crate::categories::PhrasingContent for Progress {}
impl crate::categories::PalpableContent for Progress {}
