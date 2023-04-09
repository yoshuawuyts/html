/// The HTML `<h6>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h6)
#[doc(alias = "h6")]
#[non_exhaustive]
pub struct Heading6<T: crate::categories::PhrasingContent> {
    _sys: html_sys::sections::Heading6,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading6<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading6<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading6<T> {}
