/// The HTML `<h3>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3)
#[doc(alias = "h3")]
#[non_exhaustive]
pub struct Heading3 {
    _sys: html_sys::sections::Heading3,
}
impl crate::categories::FlowContent for Heading3 {}
impl crate::categories::HeadingContent for Heading3 {}
impl crate::categories::PalpableContent for Heading3 {}
