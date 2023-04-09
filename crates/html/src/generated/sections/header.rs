/// The HTML `<header>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/header)
#[doc(alias = "header")]
#[non_exhaustive]
pub struct Header {
    sys: html_sys::sections::Header,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Header {}
impl crate::categories::PalpableContent for Header {}
impl std::convert::Into<html_sys::sections::Header> for Header {
    fn into(self) -> html_sys::sections::Header {
        self.sys
    }
}
impl From<html_sys::sections::Header> for Header {
    fn from(sys: html_sys::sections::Header) -> Self {
        Self { sys, _children: vec![] }
    }
}
