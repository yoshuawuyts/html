/// The HTML `<dd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
#[doc(alias = "dd")]
#[non_exhaustive]
pub struct DescriptionDetails<T: crate::categories::FlowContent> {
    sys: html_sys::text::DescriptionDetails,
    _children: Vec<T>,
}
impl<
    T: crate::categories::FlowContent,
> std::convert::Into<html_sys::text::DescriptionDetails> for DescriptionDetails<T> {
    fn into(self) -> html_sys::text::DescriptionDetails {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::text::DescriptionDetails>
for DescriptionDetails<T> {
    fn from(sys: html_sys::text::DescriptionDetails) -> Self {
        Self { sys, _children: vec![] }
    }
}
