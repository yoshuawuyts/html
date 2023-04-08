/// The HTML `<header>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/header)
#[doc(alias = "header")]
#[non_exhaustive]
pub struct Header {
    _sys: html_sys::sections::Header,
}
impl crate::categories::FlowContent for Header {}
impl crate::categories::PalpableContent for Header {}
