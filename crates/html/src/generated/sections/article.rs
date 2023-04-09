/// The HTML `<article>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
#[doc(alias = "article")]
#[non_exhaustive]
pub struct Article<T: crate::categories::FlowContent> {
    _sys: html_sys::sections::Article,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Article<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Article<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for Article<T> {}
