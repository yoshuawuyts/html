/// The HTML `<bdo>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
#[doc(alias = "bdo")]
#[non_exhaustive]
pub struct BidirectionalTextOverride<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::BidirectionalTextOverride,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for BidirectionalTextOverride<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for BidirectionalTextOverride<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for BidirectionalTextOverride<T> {}
