/// The HTML `<div>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
#[doc(alias = "div")]
#[non_exhaustive]
pub struct Division {
    _sys: html_sys::text::Division,
}
impl crate::categories::FlowContent for Division {}
impl crate::categories::PalpableContent for Division {}
