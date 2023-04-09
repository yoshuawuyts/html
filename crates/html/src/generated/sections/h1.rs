/// The HTML `<h1>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1)
#[doc(alias = "h1")]
#[non_exhaustive]
pub struct Heading1 {
    sys: html_sys::sections::Heading1,
    _children: Vec<()>,
}
impl crate::HtmlElement for Heading1 {}
impl crate::categories::FlowContent for Heading1 {}
impl crate::categories::HeadingContent for Heading1 {}
impl crate::categories::PalpableContent for Heading1 {}
impl std::convert::Into<html_sys::sections::Heading1> for Heading1 {
    fn into(self) -> html_sys::sections::Heading1 {
        self.sys
    }
}
impl From<html_sys::sections::Heading1> for Heading1 {
    fn from(sys: html_sys::sections::Heading1) -> Self {
        Self { sys, _children: vec![] }
    }
}
