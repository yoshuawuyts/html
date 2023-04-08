/// The HTML `<h4>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h4)
#[doc(alias = "h4")]
#[non_exhaustive]
pub struct Heading4 {
    _sys: html_sys::sections::Heading4,
}
impl crate::categories::FlowContent for Heading4 {}
impl crate::categories::HeadingContent for Heading4 {}
impl crate::categories::PalpableContent for Heading4 {}
