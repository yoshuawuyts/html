/// The HTML `<iframe>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
#[doc(alias = "iframe")]
#[non_exhaustive]
pub struct Iframe {
    sys: html_sys::embedded::Iframe,
}
impl Iframe {
    /// Get the value of the `src` attribute
    pub fn src(&self) -> std::option::Option<&str> {
        self.sys.src.as_deref()
    }
    /// Set the value of the `src` attribute
    pub fn set_src(&mut self, value: std::option::Option<String>) {
        self.sys.src = value;
    }
    /// Get the value of the `srcdoc` attribute
    pub fn srcdoc(&self) -> std::option::Option<&str> {
        self.sys.srcdoc.as_deref()
    }
    /// Set the value of the `srcdoc` attribute
    pub fn set_srcdoc(&mut self, value: std::option::Option<String>) {
        self.sys.srcdoc = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `sandbox` attribute
    pub fn sandbox(&self) -> std::option::Option<&str> {
        self.sys.sandbox.as_deref()
    }
    /// Set the value of the `sandbox` attribute
    pub fn set_sandbox(&mut self, value: std::option::Option<String>) {
        self.sys.sandbox = value;
    }
    /// Get the value of the `allow` attribute
    pub fn allow(&self) -> std::option::Option<&str> {
        self.sys.allow.as_deref()
    }
    /// Set the value of the `allow` attribute
    pub fn set_allow(&mut self, value: std::option::Option<String>) {
        self.sys.allow = value;
    }
    /// Get the value of the `allowfullscreen` attribute
    pub fn allowfullscreen(&self) -> std::option::Option<&str> {
        self.sys.allowfullscreen.as_deref()
    }
    /// Set the value of the `allowfullscreen` attribute
    pub fn set_allowfullscreen(&mut self, value: std::option::Option<String>) {
        self.sys.allowfullscreen = value;
    }
    /// Get the value of the `width` attribute
    pub fn width(&self) -> std::option::Option<&str> {
        self.sys.width.as_deref()
    }
    /// Set the value of the `width` attribute
    pub fn set_width(&mut self, value: std::option::Option<String>) {
        self.sys.width = value;
    }
    /// Get the value of the `height` attribute
    pub fn height(&self) -> std::option::Option<&str> {
        self.sys.height.as_deref()
    }
    /// Set the value of the `height` attribute
    pub fn set_height(&mut self, value: std::option::Option<String>) {
        self.sys.height = value;
    }
    /// Get the value of the `referrerpolicy` attribute
    pub fn referrerpolicy(&self) -> std::option::Option<&str> {
        self.sys.referrerpolicy.as_deref()
    }
    /// Set the value of the `referrerpolicy` attribute
    pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
        self.sys.referrerpolicy = value;
    }
    /// Get the value of the `loading` attribute
    pub fn loading(&self) -> std::option::Option<&str> {
        self.sys.loading.as_deref()
    }
    /// Set the value of the `loading` attribute
    pub fn set_loading(&mut self, value: std::option::Option<String>) {
        self.sys.loading = value;
    }
}
impl crate::categories::FlowContent for Iframe {}
impl crate::categories::PhrasingContent for Iframe {}
impl crate::categories::EmbeddedContent for Iframe {}
impl crate::categories::InteractiveContent for Iframe {}
impl crate::categories::PalpableContent for Iframe {}
impl std::convert::Into<html_sys::embedded::Iframe> for Iframe {
    fn into(self) -> html_sys::embedded::Iframe {
        self.sys
    }
}
impl From<html_sys::embedded::Iframe> for Iframe {
    fn from(sys: html_sys::embedded::Iframe) -> Self {
        Self { sys }
    }
}
