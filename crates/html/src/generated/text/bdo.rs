/// The HTML `<bdo>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
#[doc(alias = "bdo")]
#[non_exhaustive]
pub struct BidirectionalTextOverride {
    sys: html_sys::text::BidirectionalTextOverride,
    _children: Vec<()>,
}
impl crate::HtmlElement for BidirectionalTextOverride {}
impl crate::categories::FlowContent for BidirectionalTextOverride {}
impl crate::categories::PhrasingContent for BidirectionalTextOverride {}
impl crate::categories::PalpableContent for BidirectionalTextOverride {}
impl std::convert::Into<html_sys::text::BidirectionalTextOverride>
for BidirectionalTextOverride {
    fn into(self) -> html_sys::text::BidirectionalTextOverride {
        self.sys
    }
}
impl From<html_sys::text::BidirectionalTextOverride> for BidirectionalTextOverride {
    fn from(sys: html_sys::text::BidirectionalTextOverride) -> Self {
        Self { sys, _children: vec![] }
    }
}
