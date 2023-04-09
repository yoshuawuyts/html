/// The HTML `<u>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
#[doc(alias = "u")]
#[non_exhaustive]
pub struct Underline<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Underline,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Underline<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Underline<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Underline<T> {}
