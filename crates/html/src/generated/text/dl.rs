/// The HTML `<dl>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl)
#[doc(alias = "dl")]
#[non_exhaustive]
pub struct DescriptionList {
    sys: html_sys::text::DescriptionList,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for DescriptionList {}
impl std::convert::Into<html_sys::text::DescriptionList> for DescriptionList {
    fn into(self) -> html_sys::text::DescriptionList {
        self.sys
    }
}
impl From<html_sys::text::DescriptionList> for DescriptionList {
    fn from(sys: html_sys::text::DescriptionList) -> Self {
        Self { sys, _children: vec![] }
    }
}
