/// The HTML `<mark>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
#[doc(alias = "mark")]
#[non_exhaustive]
pub struct MarkText<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::MarkText,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for MarkText<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for MarkText<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for MarkText<T> {}
