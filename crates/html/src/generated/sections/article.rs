/// The HTML `<article>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
#[doc(alias = "article")]
#[non_exhaustive]
pub struct Article {
    sys: html_sys::sections::Article,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Article {}
impl crate::categories::SectioningContent for Article {}
impl crate::categories::PalpableContent for Article {}
impl std::convert::Into<html_sys::sections::Article> for Article {
    fn into(self) -> html_sys::sections::Article {
        self.sys
    }
}
impl From<html_sys::sections::Article> for Article {
    fn from(sys: html_sys::sections::Article) -> Self {
        Self { sys, _children: vec![] }
    }
}
