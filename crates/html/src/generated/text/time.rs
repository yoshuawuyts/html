/// The HTML `<time>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
#[doc(alias = "time")]
#[non_exhaustive]
pub struct Time {
    _sys: html_sys::text::Time,
}
impl crate::categories::FlowContent for Time {}
impl crate::categories::PhrasingContent for Time {}
impl crate::categories::PalpableContent for Time {}
