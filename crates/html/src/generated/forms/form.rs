/// The HTML `<form>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
#[doc(alias = "form")]
#[non_exhaustive]
pub struct Form {
    _sys: html_sys::forms::Form,
}
impl crate::categories::FlowContent for Form {}
impl crate::categories::PalpableContent for Form {}
