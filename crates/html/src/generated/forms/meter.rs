/// The HTML `<meter>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
#[doc(alias = "meter")]
#[non_exhaustive]
pub struct Meter {
    _sys: html_sys::forms::Meter,
}
impl crate::categories::FlowContent for Meter {}
impl crate::categories::PhrasingContent for Meter {}
impl crate::categories::PalpableContent for Meter {}
