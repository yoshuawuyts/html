/// The HTML `<nav>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
#[doc(alias = "nav")]
#[non_exhaustive]
pub struct Navigation {
    sys: html_sys::sections::Navigation,
    _children: Vec<()>,
}
impl crate::HtmlElement for Navigation {}
impl crate::categories::FlowContent for Navigation {}
impl crate::categories::SectioningContent for Navigation {}
impl crate::categories::PalpableContent for Navigation {}
impl std::convert::Into<html_sys::sections::Navigation> for Navigation {
    fn into(self) -> html_sys::sections::Navigation {
        self.sys
    }
}
impl From<html_sys::sections::Navigation> for Navigation {
    fn from(sys: html_sys::sections::Navigation) -> Self {
        Self { sys, _children: vec![] }
    }
}
