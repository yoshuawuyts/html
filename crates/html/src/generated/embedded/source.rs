/// The HTML `<source>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
#[doc(alias = "source")]
#[non_exhaustive]
pub struct MediaSource {
    sys: html_sys::embedded::MediaSource,
}
impl MediaSource {
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
}
impl crate::HtmlElement for MediaSource {}
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
