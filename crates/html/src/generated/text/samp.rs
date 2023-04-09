/// The HTML `<samp>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
#[doc(alias = "samp")]
#[non_exhaustive]
pub struct SampleOutput<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::SampleOutput,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SampleOutput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SampleOutput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SampleOutput<T> {}
