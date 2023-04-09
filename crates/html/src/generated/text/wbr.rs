/// The HTML `<wbr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/wbr)
#[doc(alias = "wbr")]
#[non_exhaustive]
pub struct LineBreakOpportunity {
    sys: html_sys::text::LineBreakOpportunity,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for LineBreakOpportunity {}
impl crate::categories::PhrasingContent for LineBreakOpportunity {}
impl std::convert::Into<html_sys::text::LineBreakOpportunity> for LineBreakOpportunity {
    fn into(self) -> html_sys::text::LineBreakOpportunity {
        self.sys
    }
}
impl From<html_sys::text::LineBreakOpportunity> for LineBreakOpportunity {
    fn from(sys: html_sys::text::LineBreakOpportunity) -> Self {
        Self { sys, _children: vec![] }
    }
}
