/// The HTML `<bdi>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
#[doc(alias = "bdi")]
#[non_exhaustive]
pub struct BidirectionalIsolate {
    _sys: html_sys::text::BidirectionalIsolate,
}
impl crate::categories::FlowContent for BidirectionalIsolate {}
impl crate::categories::PhrasingContent for BidirectionalIsolate {}
impl crate::categories::PalpableContent for BidirectionalIsolate {}
