/// The HTML `<details>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
#[doc(alias = "details")]
#[non_exhaustive]
pub struct Details {
    _sys: html_sys::interactive::Details,
}
impl crate::categories::FlowContent for Details {}
impl crate::categories::InteractiveContent for Details {}
impl crate::categories::PalpableContent for Details {}
