/// The HTML `<a>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
#[doc(alias = "a")]
#[non_exhaustive]
pub struct Anchor {
    sys: html_sys::text::Anchor,
    _children: Vec<()>,
}
impl Anchor {
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
    /// Get the value of the `download` attribute
    pub fn download(&self) -> std::option::Option<&str> {
        self.sys.download.as_deref()
    }
    /// Set the value of the `download` attribute
    pub fn set_download(&mut self, value: std::option::Option<String>) {
        self.sys.download = value;
    }
    /// Get the value of the `ping` attribute
    pub fn ping(&self) -> std::option::Option<&str> {
        self.sys.ping.as_deref()
    }
    /// Set the value of the `ping` attribute
    pub fn set_ping(&mut self, value: std::option::Option<String>) {
        self.sys.ping = value;
    }
    /// Get the value of the `rel` attribute
    pub fn rel(&self) -> std::option::Option<&str> {
        self.sys.rel.as_deref()
    }
    /// Set the value of the `rel` attribute
    pub fn set_rel(&mut self, value: std::option::Option<String>) {
        self.sys.rel = value;
    }
    /// Get the value of the `hreflang` attribute
    pub fn hreflang(&self) -> std::option::Option<&str> {
        self.sys.hreflang.as_deref()
    }
    /// Set the value of the `hreflang` attribute
    pub fn set_hreflang(&mut self, value: std::option::Option<String>) {
        self.sys.hreflang = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
    /// Get the value of the `referrerpolicy` attribute
    pub fn referrerpolicy(&self) -> std::option::Option<&str> {
        self.sys.referrerpolicy.as_deref()
    }
    /// Set the value of the `referrerpolicy` attribute
    pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
        self.sys.referrerpolicy = value;
    }
}
impl crate::HtmlElement for Anchor {}
impl crate::categories::FlowContent for Anchor {}
impl crate::categories::PhrasingContent for Anchor {}
impl crate::categories::PalpableContent for Anchor {}
impl std::convert::Into<html_sys::text::Anchor> for Anchor {
    fn into(self) -> html_sys::text::Anchor {
        self.sys
    }
}
impl From<html_sys::text::Anchor> for Anchor {
    fn from(sys: html_sys::text::Anchor) -> Self {
        Self { sys, _children: vec![] }
    }
}
