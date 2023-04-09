/// The HTML `<hgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
#[doc(alias = "hgroup")]
#[non_exhaustive]
pub struct HeadingGroup {
    sys: html_sys::sections::HeadingGroup,
    _children: Vec<()>,
}
impl crate::HtmlElement for HeadingGroup {}
impl crate::categories::FlowContent for HeadingGroup {}
impl crate::categories::HeadingContent for HeadingGroup {}
impl crate::categories::PalpableContent for HeadingGroup {}
impl std::convert::Into<html_sys::sections::HeadingGroup> for HeadingGroup {
    fn into(self) -> html_sys::sections::HeadingGroup {
        self.sys
    }
}
impl From<html_sys::sections::HeadingGroup> for HeadingGroup {
    fn from(sys: html_sys::sections::HeadingGroup) -> Self {
        Self { sys, _children: vec![] }
    }
}
