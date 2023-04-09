/// The HTML `<main>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main)
#[doc(alias = "main")]
#[non_exhaustive]
pub struct Main {
    sys: html_sys::text::Main,
    _children: Vec<()>,
}
impl crate::HtmlElement for Main {}
impl crate::categories::FlowContent for Main {}
impl crate::categories::PalpableContent for Main {}
impl std::convert::Into<html_sys::text::Main> for Main {
    fn into(self) -> html_sys::text::Main {
        self.sys
    }
}
impl From<html_sys::text::Main> for Main {
    fn from(sys: html_sys::text::Main) -> Self {
        Self { sys, _children: vec![] }
    }
}
