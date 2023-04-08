/// The HTML `<small>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
#[doc(alias = "small")]
#[non_exhaustive]
pub struct SideComment {
    _sys: html_sys::text::SideComment,
}
impl crate::categories::FlowContent for SideComment {}
impl crate::categories::PhrasingContent for SideComment {}
impl crate::categories::PalpableContent for SideComment {}
