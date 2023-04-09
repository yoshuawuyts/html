/// The HTML `<nav>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
#[doc(alias = "nav")]
#[non_exhaustive]
pub struct Navigation<T: crate::categories::FlowContent> {
    sys: html_sys::sections::Navigation,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent
for Navigation<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Navigation<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for Navigation<T> {}
impl<
    T: crate::categories::FlowContent,
> std::convert::Into<html_sys::sections::Navigation> for Navigation<T> {
    fn into(self) -> html_sys::sections::Navigation {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::sections::Navigation>
for Navigation<T> {
    fn from(sys: html_sys::sections::Navigation) -> Self {
        Self { sys, _children: vec![] }
    }
}
