/// The HTML `<section>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
#[doc(alias = "section")]
#[non_exhaustive]
pub struct Section<T: crate::categories::FlowContent> {
    _sys: html_sys::sections::Section,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Section<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Section<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for Section<T> {}
