/// The HTML `<em>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
#[doc(alias = "em")]
#[non_exhaustive]
pub struct Emphasis<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Emphasis,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Emphasis<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Emphasis<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Emphasis<T> {}
