/// The HTML `<samp>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
#[doc(alias = "samp")]
#[non_exhaustive]
pub struct SampleOutput {
    sys: html_sys::text::SampleOutput,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for SampleOutput {}
impl crate::categories::PhrasingContent for SampleOutput {}
impl crate::categories::PalpableContent for SampleOutput {}
impl std::convert::Into<html_sys::text::SampleOutput> for SampleOutput {
    fn into(self) -> html_sys::text::SampleOutput {
        self.sys
    }
}
impl From<html_sys::text::SampleOutput> for SampleOutput {
    fn from(sys: html_sys::text::SampleOutput) -> Self {
        Self { sys, _children: vec![] }
    }
}
