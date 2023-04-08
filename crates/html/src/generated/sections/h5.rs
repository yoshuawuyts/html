/// The HTML `<h5>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
#[doc(alias = "h5")]
#[non_exhaustive]
pub struct Heading5 {
    _sys: html_sys::sections::Heading5,
}
impl crate::categories::FlowContent for Heading5 {}
impl crate::categories::HeadingContent for Heading5 {}
impl crate::categories::PalpableContent for Heading5 {}
