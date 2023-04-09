/// The HTML `<h2>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
#[doc(alias = "h2")]
#[non_exhaustive]
pub struct Heading2<T: crate::categories::PhrasingContent> {
    _sys: html_sys::sections::Heading2,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading2<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading2<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading2<T> {}
