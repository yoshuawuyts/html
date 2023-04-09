/// The HTML `<aside>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
#[doc(alias = "aside")]
#[non_exhaustive]
pub struct Aside<T: crate::categories::FlowContent> {
    sys: html_sys::sections::Aside,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Aside<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Aside<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent for Aside<T> {}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::sections::Aside>
for Aside<T> {
    fn into(self) -> html_sys::sections::Aside {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::sections::Aside> for Aside<T> {
    fn from(sys: html_sys::sections::Aside) -> Self {
        Self { sys, _children: vec![] }
    }
}
