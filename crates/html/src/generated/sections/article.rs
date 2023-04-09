/// The HTML `<article>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
#[doc(alias = "article")]
#[non_exhaustive]
pub struct Article<T: crate::categories::FlowContent> {
    sys: html_sys::sections::Article,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Article<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Article<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for Article<T> {}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::sections::Article>
for Article<T> {
    fn into(self) -> html_sys::sections::Article {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::sections::Article>
for Article<T> {
    fn from(sys: html_sys::sections::Article) -> Self {
        Self { sys, _children: vec![] }
    }
}
