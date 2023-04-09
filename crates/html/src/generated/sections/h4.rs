/// The HTML `<h4>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h4)
#[doc(alias = "h4")]
#[non_exhaustive]
pub struct Heading4<T: crate::categories::PhrasingContent> {
    _sys: html_sys::sections::Heading4,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading4<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading4<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading4<T> {}
