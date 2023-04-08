/// The HTML `<bdo>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
#[doc(alias = "bdo")]
#[non_exhaustive]
pub struct BidirectionalTextOverride {
    _sys: html_sys::text::BidirectionalTextOverride,
}
impl crate::categories::FlowContent for BidirectionalTextOverride {}
impl crate::categories::PhrasingContent for BidirectionalTextOverride {}
impl crate::categories::PalpableContent for BidirectionalTextOverride {}
