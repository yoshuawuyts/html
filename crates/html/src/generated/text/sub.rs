/// The HTML `<sub>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
#[doc(alias = "sub")]
#[non_exhaustive]
pub struct SubScript<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::SubScript,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SubScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SubScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SubScript<T> {}
