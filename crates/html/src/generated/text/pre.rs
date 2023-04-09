/// The HTML `<pre>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
#[doc(alias = "pre")]
#[non_exhaustive]
pub struct PreformattedText<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::PreformattedText,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for PreformattedText<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for PreformattedText<T> {}
