/// The HTML `<h2>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
#[doc(alias = "h2")]
#[non_exhaustive]
pub struct Heading2 {
    _sys: html_sys::sections::Heading2,
}
impl crate::categories::FlowContent for Heading2 {}
impl crate::categories::HeadingContent for Heading2 {}
impl crate::categories::PalpableContent for Heading2 {}
