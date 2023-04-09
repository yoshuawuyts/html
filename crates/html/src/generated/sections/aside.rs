/// The HTML `<aside>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
#[doc(alias = "aside")]
#[non_exhaustive]
pub struct Aside<T: crate::categories::FlowContent> {
    _sys: html_sys::sections::Aside,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Aside<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Aside<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent for Aside<T> {}
