/// The HTML `<audio>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
#[doc(alias = "audio")]
#[non_exhaustive]
pub struct Audio {
    _sys: html_sys::embedded::Audio,
}
impl crate::categories::FlowContent for Audio {}
impl crate::categories::PhrasingContent for Audio {}
impl crate::categories::EmbeddedContent for Audio {}
