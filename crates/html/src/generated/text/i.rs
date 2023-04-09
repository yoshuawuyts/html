/// The HTML `<i>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i)
#[doc(alias = "i")]
#[non_exhaustive]
pub struct Italic<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Italic,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Italic<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Italic<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Italic<T> {}
