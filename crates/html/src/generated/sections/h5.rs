/// The HTML `<h5>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
#[doc(alias = "h5")]
#[non_exhaustive]
pub struct Heading5<T: crate::categories::PhrasingContent> {
    _sys: html_sys::sections::Heading5,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading5<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading5<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading5<T> {}
