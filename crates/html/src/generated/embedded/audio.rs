/// The HTML `<audio>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
#[doc(alias = "audio")]
#[non_exhaustive]
pub struct Audio {
    sys: html_sys::embedded::Audio,
}
impl crate::categories::FlowContent for Audio {}
impl crate::categories::PhrasingContent for Audio {}
impl crate::categories::EmbeddedContent for Audio {}
impl std::convert::Into<html_sys::embedded::Audio> for Audio {
    fn into(self) -> html_sys::embedded::Audio {
        self.sys
    }
}
impl From<html_sys::embedded::Audio> for Audio {
    fn from(sys: html_sys::embedded::Audio) -> Self {
        Self { sys }
    }
}
