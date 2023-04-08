/// The HTML `<data>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
#[doc(alias = "data")]
#[non_exhaustive]
pub struct Data {
    _sys: html_sys::text::Data,
}
impl crate::categories::FlowContent for Data {}
impl crate::categories::PhrasingContent for Data {}
impl crate::categories::PalpableContent for Data {}
