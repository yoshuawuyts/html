/// The HTML `<samp>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
#[doc(alias = "samp")]
#[non_exhaustive]
pub struct SampleOutput<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::SampleOutput,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SampleOutput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SampleOutput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SampleOutput<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::SampleOutput> for SampleOutput<T> {
    fn into(self) -> html_sys::text::SampleOutput {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::SampleOutput>
for SampleOutput<T> {
    fn from(sys: html_sys::text::SampleOutput) -> Self {
        Self { sys, _children: vec![] }
    }
}
