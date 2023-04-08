/// The HTML `<h6>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h6)
#[doc(alias = "h6")]
#[non_exhaustive]
pub struct Heading6 {
    _sys: html_sys::sections::Heading6,
}
impl crate::categories::FlowContent for Heading6 {}
impl crate::categories::HeadingContent for Heading6 {}
impl crate::categories::PalpableContent for Heading6 {}
