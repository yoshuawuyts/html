/// The HTML `<video>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
#[doc(alias = "video")]
#[non_exhaustive]
pub struct Video {
    _sys: html_sys::embedded::Video,
}
impl crate::categories::FlowContent for Video {}
impl crate::categories::PhrasingContent for Video {}
impl crate::categories::EmbeddedContent for Video {}
impl crate::categories::PalpableContent for Video {}
