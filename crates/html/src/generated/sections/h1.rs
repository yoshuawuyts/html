/// The HTML `<h1>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1)
#[doc(alias = "h1")]
#[non_exhaustive]
pub struct Heading1<T: crate::categories::PhrasingContent> {
    _sys: html_sys::sections::Heading1,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading1<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading1<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading1<T> {}
