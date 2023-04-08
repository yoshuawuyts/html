/// The HTML `<pre>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
#[doc(alias = "pre")]
#[non_exhaustive]
pub struct PreformattedText {
    _sys: html_sys::text::PreformattedText,
}
impl crate::categories::FlowContent for PreformattedText {}
impl crate::categories::PalpableContent for PreformattedText {}
