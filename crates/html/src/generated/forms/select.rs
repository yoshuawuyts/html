/// The HTML `<select>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
#[doc(alias = "select")]
#[non_exhaustive]
pub struct Select {
    _sys: html_sys::forms::Select,
}
impl crate::categories::FlowContent for Select {}
impl crate::categories::PhrasingContent for Select {}
impl crate::categories::InteractiveContent for Select {}
impl crate::categories::PalpableContent for Select {}
