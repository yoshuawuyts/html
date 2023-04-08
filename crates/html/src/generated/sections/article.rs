/// The HTML `<article>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
#[doc(alias = "article")]
#[non_exhaustive]
pub struct Article {
    _sys: html_sys::sections::Article,
}
impl crate::categories::FlowContent for Article {}
impl crate::categories::SectioningContent for Article {}
impl crate::categories::PalpableContent for Article {}
