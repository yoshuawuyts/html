/// The HTML `<cite>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
#[doc(alias = "cite")]
#[non_exhaustive]
pub struct Cite<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Cite,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Cite<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Cite<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Cite<T> {}
