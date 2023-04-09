/// The HTML `<video>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
#[doc(alias = "video")]
#[non_exhaustive]
pub struct Video {
    sys: html_sys::embedded::Video,
}
impl Video {
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
    /// Get the value of the `poster` attribute
    pub fn poster(&self) -> std::option::Option<&str> {
        self.sys.poster.as_deref()
    }
    /// Set the value of the `poster` attribute
    pub fn set_poster(&mut self, value: std::option::Option<String>) {
        self.sys.poster = value;
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
    /// Get the value of the `playsinline` attribute
    pub fn playsinline(&self) -> std::option::Option<&str> {
        self.sys.playsinline.as_deref()
    }
    /// Set the value of the `playsinline` attribute
    pub fn set_playsinline(&mut self, value: std::option::Option<String>) {
        self.sys.playsinline = value;
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
}
impl crate::categories::FlowContent for Video {}
impl crate::categories::PhrasingContent for Video {}
impl crate::categories::EmbeddedContent for Video {}
impl crate::categories::PalpableContent for Video {}
impl std::convert::Into<html_sys::embedded::Video> for Video {
    fn into(self) -> html_sys::embedded::Video {
        self.sys
    }
}
impl From<html_sys::embedded::Video> for Video {
    fn from(sys: html_sys::embedded::Video) -> Self {
        Self { sys }
    }
}
