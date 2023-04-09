/// The HTML `<source>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
#[doc(alias = "source")]
#[non_exhaustive]
pub struct MediaSource {
    sys: html_sys::embedded::MediaSource,
}
impl std::convert::Into<html_sys::embedded::MediaSource> for MediaSource {
    fn into(self) -> html_sys::embedded::MediaSource {
        self.sys
    }
}
impl From<html_sys::embedded::MediaSource> for MediaSource {
    fn from(sys: html_sys::embedded::MediaSource) -> Self {
        Self { sys }
    }
}
