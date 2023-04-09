/// The HTML `<bdo>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
#[doc(alias = "bdo")]
#[non_exhaustive]
pub struct BidirectionalTextOverride<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::BidirectionalTextOverride,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for BidirectionalTextOverride<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for BidirectionalTextOverride<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for BidirectionalTextOverride<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::BidirectionalTextOverride>
for BidirectionalTextOverride<T> {
    fn into(self) -> html_sys::text::BidirectionalTextOverride {
        self.sys
    }
}
impl<
    T: crate::categories::PhrasingContent,
> From<html_sys::text::BidirectionalTextOverride> for BidirectionalTextOverride<T> {
    fn from(sys: html_sys::text::BidirectionalTextOverride) -> Self {
        Self { sys, _children: vec![] }
    }
}
