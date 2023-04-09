/// The HTML `<sup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
#[doc(alias = "sup")]
#[non_exhaustive]
pub struct SuperScript<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::SuperScript,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SuperScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SuperScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SuperScript<T> {}
