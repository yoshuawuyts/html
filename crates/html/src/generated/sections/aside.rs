/// The HTML `<aside>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
#[doc(alias = "aside")]
#[non_exhaustive]
pub struct Aside {
    sys: html_sys::sections::Aside,
    _children: Vec<()>,
}
impl crate::HtmlElement for Aside {}
impl crate::categories::FlowContent for Aside {}
impl crate::categories::SectioningContent for Aside {}
impl crate::categories::PalpableContent for Aside {}
impl std::convert::Into<html_sys::sections::Aside> for Aside {
    fn into(self) -> html_sys::sections::Aside {
        self.sys
    }
}
impl From<html_sys::sections::Aside> for Aside {
    fn from(sys: html_sys::sections::Aside) -> Self {
        Self { sys, _children: vec![] }
    }
}
