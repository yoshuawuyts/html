/// The HTML `<h3>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3)
#[doc(alias = "h3")]
#[non_exhaustive]
pub struct Heading3 {
    sys: html_sys::sections::Heading3,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Heading3 {}
impl crate::categories::HeadingContent for Heading3 {}
impl crate::categories::PalpableContent for Heading3 {}
impl std::convert::Into<html_sys::sections::Heading3> for Heading3 {
    fn into(self) -> html_sys::sections::Heading3 {
        self.sys
    }
}
impl From<html_sys::sections::Heading3> for Heading3 {
    fn from(sys: html_sys::sections::Heading3) -> Self {
        Self { sys, _children: vec![] }
    }
}
