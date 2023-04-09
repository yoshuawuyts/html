/// The HTML `<h3>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3)
#[doc(alias = "h3")]
#[non_exhaustive]
pub struct Heading3<T: crate::categories::PhrasingContent> {
    _sys: html_sys::sections::Heading3,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading3<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading3<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading3<T> {}
