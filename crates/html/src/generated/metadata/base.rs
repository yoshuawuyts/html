/// The HTML `<base>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
#[doc(alias = "base")]
#[non_exhaustive]
pub struct Base {
    sys: html_sys::metadata::Base,
}
impl Base {
    /// Get the value of the `href` attribute
    pub fn href(&self) -> std::option::Option<&str> {
        self.sys.href.as_deref()
    }
    /// Set the value of the `href` attribute
    pub fn set_href(&mut self, value: std::option::Option<String>) {
        self.sys.href = value;
    }
    /// Get the value of the `target` attribute
    pub fn target(&self) -> std::option::Option<&str> {
        self.sys.target.as_deref()
    }
    /// Set the value of the `target` attribute
    pub fn set_target(&mut self, value: std::option::Option<String>) {
        self.sys.target = value;
    }
}
impl crate::categories::MetadataContent for Base {}
impl std::convert::Into<html_sys::metadata::Base> for Base {
    fn into(self) -> html_sys::metadata::Base {
        self.sys
    }
}
impl From<html_sys::metadata::Base> for Base {
    fn from(sys: html_sys::metadata::Base) -> Self {
        Self { sys }
    }
}
