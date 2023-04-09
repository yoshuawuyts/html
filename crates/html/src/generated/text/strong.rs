/// The HTML `<strong>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
#[doc(alias = "strong")]
#[non_exhaustive]
pub struct Strong<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Strong,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Strong<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Strong<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Strong<T> {}
