/// The HTML `<h4>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h4)
#[doc(alias = "h4")]
#[non_exhaustive]
pub struct Heading4 {
    sys: html_sys::sections::Heading4,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Heading4 {}
impl crate::categories::HeadingContent for Heading4 {}
impl crate::categories::PalpableContent for Heading4 {}
impl std::convert::Into<html_sys::sections::Heading4> for Heading4 {
    fn into(self) -> html_sys::sections::Heading4 {
        self.sys
    }
}
impl From<html_sys::sections::Heading4> for Heading4 {
    fn from(sys: html_sys::sections::Heading4) -> Self {
        Self { sys, _children: vec![] }
    }
}
