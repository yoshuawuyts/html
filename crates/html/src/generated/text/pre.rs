/// The HTML `<pre>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
#[doc(alias = "pre")]
#[non_exhaustive]
pub struct PreformattedText<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::PreformattedText,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for PreformattedText<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for PreformattedText<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::PreformattedText> for PreformattedText<T> {
    fn into(self) -> html_sys::text::PreformattedText {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::PreformattedText>
for PreformattedText<T> {
    fn from(sys: html_sys::text::PreformattedText) -> Self {
        Self { sys, _children: vec![] }
    }
}
