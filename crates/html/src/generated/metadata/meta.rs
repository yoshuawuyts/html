/// The HTML `<meta>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
#[doc(alias = "meta")]
#[non_exhaustive]
pub struct Meta {
    sys: html_sys::metadata::Meta,
    _children: Vec<()>,
}
impl Meta {
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `http-equiv` attribute
    pub fn http_equiv(&self) -> std::option::Option<&str> {
        self.sys.http_equiv.as_deref()
    }
    /// Set the value of the `http-equiv` attribute
    pub fn set_http_equiv(&mut self, value: std::option::Option<String>) {
        self.sys.http_equiv = value;
    }
    /// Get the value of the `content` attribute
    pub fn content(&self) -> std::option::Option<&str> {
        self.sys.content.as_deref()
    }
    /// Set the value of the `content` attribute
    pub fn set_content(&mut self, value: std::option::Option<String>) {
        self.sys.content = value;
    }
    /// Get the value of the `charset` attribute
    pub fn charset(&self) -> std::option::Option<&str> {
        self.sys.charset.as_deref()
    }
    /// Set the value of the `charset` attribute
    pub fn set_charset(&mut self, value: std::option::Option<String>) {
        self.sys.charset = value;
    }
    /// Get the value of the `media` attribute
    pub fn media(&self) -> std::option::Option<&str> {
        self.sys.media.as_deref()
    }
    /// Set the value of the `media` attribute
    pub fn set_media(&mut self, value: std::option::Option<String>) {
        self.sys.media = value;
    }
}
impl crate::HtmlElement for Meta {}
impl crate::categories::MetadataContent for Meta {}
impl std::convert::Into<html_sys::metadata::Meta> for Meta {
    fn into(self) -> html_sys::metadata::Meta {
        self.sys
    }
}
impl From<html_sys::metadata::Meta> for Meta {
    fn from(sys: html_sys::metadata::Meta) -> Self {
        Self { sys, _children: vec![] }
    }
}
