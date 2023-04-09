/// The HTML `<span>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
#[doc(alias = "span")]
#[non_exhaustive]
pub struct Span<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Span,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Span<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Span<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Span<T> {}
