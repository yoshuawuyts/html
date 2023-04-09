/// The HTML `<audio>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
#[doc(alias = "audio")]
#[non_exhaustive]
pub struct Audio {
    sys: html_sys::embedded::Audio,
}
impl Audio {
    /// Get the value of the `src` attribute
    pub fn src(&self) -> std::option::Option<&str> {
        self.sys.src.as_deref()
    }
    /// Set the value of the `src` attribute
    pub fn set_src(&mut self, value: std::option::Option<String>) {
        self.sys.src = value;
    }
    /// Get the value of the `crossorigin` attribute
    pub fn crossorigin(&self) -> std::option::Option<&str> {
        self.sys.crossorigin.as_deref()
    }
    /// Set the value of the `crossorigin` attribute
    pub fn set_crossorigin(&mut self, value: std::option::Option<String>) {
        self.sys.crossorigin = value;
    }
    /// Get the value of the `preload` attribute
    pub fn preload(&self) -> std::option::Option<&str> {
        self.sys.preload.as_deref()
    }
    /// Set the value of the `preload` attribute
    pub fn set_preload(&mut self, value: std::option::Option<String>) {
        self.sys.preload = value;
    }
    /// Get the value of the `autoplay` attribute
    pub fn autoplay(&self) -> std::option::Option<&str> {
        self.sys.autoplay.as_deref()
    }
    /// Set the value of the `autoplay` attribute
    pub fn set_autoplay(&mut self, value: std::option::Option<String>) {
        self.sys.autoplay = value;
    }
    /// Get the value of the `loop` attribute
    pub fn loop_(&self) -> std::option::Option<&str> {
        self.sys.loop_.as_deref()
    }
    /// Set the value of the `loop` attribute
    pub fn set_loop_(&mut self, value: std::option::Option<String>) {
        self.sys.loop_ = value;
    }
    /// Get the value of the `muted` attribute
    pub fn muted(&self) -> std::option::Option<&str> {
        self.sys.muted.as_deref()
    }
    /// Set the value of the `muted` attribute
    pub fn set_muted(&mut self, value: std::option::Option<String>) {
        self.sys.muted = value;
    }
    /// Get the value of the `controls` attribute
    pub fn controls(&self) -> std::option::Option<&str> {
        self.sys.controls.as_deref()
    }
    /// Set the value of the `controls` attribute
    pub fn set_controls(&mut self, value: std::option::Option<String>) {
        self.sys.controls = value;
    }
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
