/// The HTML `<hgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
#[doc(alias = "hgroup")]
#[non_exhaustive]
pub struct HeadingGroup {
    _sys: html_sys::sections::HeadingGroup,
}
impl crate::categories::FlowContent for HeadingGroup {}
impl crate::categories::HeadingContent for HeadingGroup {}
impl crate::categories::PalpableContent for HeadingGroup {}
