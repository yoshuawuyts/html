/// The HTML `<bdi>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
#[doc(alias = "bdi")]
#[non_exhaustive]
pub struct BidirectionalIsolate<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::BidirectionalIsolate,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for BidirectionalIsolate<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for BidirectionalIsolate<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for BidirectionalIsolate<T> {}
