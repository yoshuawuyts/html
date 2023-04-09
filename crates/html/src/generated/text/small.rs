/// The HTML `<small>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
#[doc(alias = "small")]
#[non_exhaustive]
pub struct SideComment<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::SideComment,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SideComment<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SideComment<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SideComment<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::SideComment> for SideComment<T> {
    fn into(self) -> html_sys::text::SideComment {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::SideComment>
for SideComment<T> {
    fn from(sys: html_sys::text::SideComment) -> Self {
        Self { sys, _children: vec![] }
    }
}
