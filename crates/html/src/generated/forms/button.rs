/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
pub struct Button {
    _sys: html_sys::forms::Button,
}
impl crate::categories::FlowContent for Button {}
impl crate::categories::PhrasingContent for Button {}
impl crate::categories::InteractiveContent for Button {}
impl crate::categories::PalpableContent for Button {}
