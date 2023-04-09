/// The HTML `<small>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
#[doc(alias = "small")]
#[non_exhaustive]
pub struct SideComment {
    sys: html_sys::text::SideComment,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for SideComment {}
impl crate::categories::PhrasingContent for SideComment {}
impl crate::categories::PalpableContent for SideComment {}
impl std::convert::Into<html_sys::text::SideComment> for SideComment {
    fn into(self) -> html_sys::text::SideComment {
        self.sys
    }
}
impl From<html_sys::text::SideComment> for SideComment {
    fn from(sys: html_sys::text::SideComment) -> Self {
        Self { sys, _children: vec![] }
    }
}
