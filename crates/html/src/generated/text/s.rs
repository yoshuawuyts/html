/// The HTML `<s>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
#[doc(alias = "s")]
#[non_exhaustive]
pub struct StrikeThrough<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::StrikeThrough,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for StrikeThrough<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for StrikeThrough<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for StrikeThrough<T> {}
