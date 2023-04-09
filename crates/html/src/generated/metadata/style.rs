/// The HTML `<style>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
#[doc(alias = "style")]
#[non_exhaustive]
pub struct Style {
    sys: html_sys::metadata::Style,
    _children: Vec<()>,
}
impl Style {
    /// Get the value of the `media` attribute
    pub fn media(&self) -> std::option::Option<&str> {
        self.sys.media.as_deref()
    }
    /// Set the value of the `media` attribute
    pub fn set_media(&mut self, value: std::option::Option<String>) {
        self.sys.media = value;
    }
    /// Get the value of the `blocking` attribute
    pub fn blocking(&self) -> std::option::Option<&str> {
        self.sys.blocking.as_deref()
    }
    /// Set the value of the `blocking` attribute
    pub fn set_blocking(&mut self, value: std::option::Option<String>) {
        self.sys.blocking = value;
    }
}
impl crate::categories::MetadataContent for Style {}
impl std::convert::Into<html_sys::metadata::Style> for Style {
    fn into(self) -> html_sys::metadata::Style {
        self.sys
    }
}
impl From<html_sys::metadata::Style> for Style {
    fn from(sys: html_sys::metadata::Style) -> Self {
        Self { sys, _children: vec![] }
    }
}
