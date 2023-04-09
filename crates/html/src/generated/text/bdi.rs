/// The HTML `<bdi>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
#[doc(alias = "bdi")]
#[non_exhaustive]
pub struct BidirectionalIsolate {
    sys: html_sys::text::BidirectionalIsolate,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for BidirectionalIsolate {}
impl crate::categories::PhrasingContent for BidirectionalIsolate {}
impl crate::categories::PalpableContent for BidirectionalIsolate {}
impl std::convert::Into<html_sys::text::BidirectionalIsolate> for BidirectionalIsolate {
    fn into(self) -> html_sys::text::BidirectionalIsolate {
        self.sys
    }
}
impl From<html_sys::text::BidirectionalIsolate> for BidirectionalIsolate {
    fn from(sys: html_sys::text::BidirectionalIsolate) -> Self {
        Self { sys, _children: vec![] }
    }
}
