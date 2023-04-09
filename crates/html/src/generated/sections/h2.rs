/// The HTML `<h2>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
#[doc(alias = "h2")]
#[non_exhaustive]
pub struct Heading2 {
    sys: html_sys::sections::Heading2,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Heading2 {}
impl crate::categories::HeadingContent for Heading2 {}
impl crate::categories::PalpableContent for Heading2 {}
impl std::convert::Into<html_sys::sections::Heading2> for Heading2 {
    fn into(self) -> html_sys::sections::Heading2 {
        self.sys
    }
}
impl From<html_sys::sections::Heading2> for Heading2 {
    fn from(sys: html_sys::sections::Heading2) -> Self {
        Self { sys, _children: vec![] }
    }
}
