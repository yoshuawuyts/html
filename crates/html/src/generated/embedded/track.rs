/// The HTML `<track>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
#[doc(alias = "track")]
#[non_exhaustive]
pub struct TextTrack {
    sys: html_sys::embedded::TextTrack,
}
impl std::convert::Into<html_sys::embedded::TextTrack> for TextTrack {
    fn into(self) -> html_sys::embedded::TextTrack {
        self.sys
    }
}
impl From<html_sys::embedded::TextTrack> for TextTrack {
    fn from(sys: html_sys::embedded::TextTrack) -> Self {
        Self { sys }
    }
}
