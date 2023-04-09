/// The HTML `<output>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
#[doc(alias = "output")]
#[non_exhaustive]
pub struct Output<T: crate::categories::PhrasingContent> {
    _sys: html_sys::forms::Output,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Output<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Output<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Output<T> {}
