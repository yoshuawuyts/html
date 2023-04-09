/// The HTML `<h5>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
#[doc(alias = "h5")]
#[non_exhaustive]
pub struct Heading5 {
    sys: html_sys::sections::Heading5,
    _children: Vec<()>,
}
impl crate::HtmlElement for Heading5 {}
impl crate::categories::FlowContent for Heading5 {}
impl crate::categories::HeadingContent for Heading5 {}
impl crate::categories::PalpableContent for Heading5 {}
impl std::convert::Into<html_sys::sections::Heading5> for Heading5 {
    fn into(self) -> html_sys::sections::Heading5 {
        self.sys
    }
}
impl From<html_sys::sections::Heading5> for Heading5 {
    fn from(sys: html_sys::sections::Heading5) -> Self {
        Self { sys, _children: vec![] }
    }
}
