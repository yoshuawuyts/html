/// The HTML `<h1>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1)
#[doc(alias = "h1")]
#[non_exhaustive]
pub struct Heading1 {
    _sys: html_sys::sections::Heading1,
}
impl crate::categories::FlowContent for Heading1 {}
impl crate::categories::HeadingContent for Heading1 {}
impl crate::categories::PalpableContent for Heading1 {}
