/// The HTML `<s>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
#[doc(alias = "s")]
#[non_exhaustive]
pub struct StrikeThrough {
    _sys: html_sys::text::StrikeThrough,
}
impl crate::categories::FlowContent for StrikeThrough {}
impl crate::categories::PhrasingContent for StrikeThrough {}
impl crate::categories::PalpableContent for StrikeThrough {}
