/// The HTML `<p>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
#[doc(alias = "p")]
#[non_exhaustive]
pub struct Paragraph<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Paragraph,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Paragraph<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Paragraph<T> {}
