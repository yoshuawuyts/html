/// The HTML `<h6>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h6)
#[doc(alias = "h6")]
#[non_exhaustive]
pub struct Heading6 {
    sys: html_sys::sections::Heading6,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Heading6 {}
impl crate::categories::HeadingContent for Heading6 {}
impl crate::categories::PalpableContent for Heading6 {}
impl std::convert::Into<html_sys::sections::Heading6> for Heading6 {
    fn into(self) -> html_sys::sections::Heading6 {
        self.sys
    }
}
impl From<html_sys::sections::Heading6> for Heading6 {
    fn from(sys: html_sys::sections::Heading6) -> Self {
        Self { sys, _children: vec![] }
    }
}
