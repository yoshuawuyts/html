/// The HTML `<img>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
#[doc(alias = "img")]
#[non_exhaustive]
pub struct Image {
    sys: html_sys::embedded::Image,
    _children: Vec<()>,
}
impl Image {
    /// Get the value of the `alt` attribute
    pub fn alt(&self) -> std::option::Option<&str> {
        self.sys.alt.as_deref()
    }
    /// Set the value of the `alt` attribute
    pub fn set_alt(&mut self, value: std::option::Option<String>) {
        self.sys.alt = value;
    }
    /// Get the value of the `src` attribute
    pub fn src(&self) -> std::option::Option<&str> {
        self.sys.src.as_deref()
    }
    /// Set the value of the `src` attribute
    pub fn set_src(&mut self, value: std::option::Option<String>) {
        self.sys.src = value;
    }
    /// Get the value of the `srcset` attribute
    pub fn srcset(&self) -> std::option::Option<&str> {
        self.sys.srcset.as_deref()
    }
    /// Set the value of the `srcset` attribute
    pub fn set_srcset(&mut self, value: std::option::Option<String>) {
        self.sys.srcset = value;
    }
    /// Get the value of the `sizes` attribute
    pub fn sizes(&self) -> std::option::Option<&str> {
        self.sys.sizes.as_deref()
    }
    /// Set the value of the `sizes` attribute
    pub fn set_sizes(&mut self, value: std::option::Option<String>) {
        self.sys.sizes = value;
    }
    /// Get the value of the `crossorigin` attribute
    pub fn crossorigin(&self) -> std::option::Option<&str> {
        self.sys.crossorigin.as_deref()
    }
    /// Set the value of the `crossorigin` attribute
    pub fn set_crossorigin(&mut self, value: std::option::Option<String>) {
        self.sys.crossorigin = value;
    }
    /// Get the value of the `usemap` attribute
    pub fn usemap(&self) -> std::option::Option<&str> {
        self.sys.usemap.as_deref()
    }
    /// Set the value of the `usemap` attribute
    pub fn set_usemap(&mut self, value: std::option::Option<String>) {
        self.sys.usemap = value;
    }
    /// Get the value of the `ismap` attribute
    pub fn ismap(&self) -> std::option::Option<&str> {
        self.sys.ismap.as_deref()
    }
    /// Set the value of the `ismap` attribute
    pub fn set_ismap(&mut self, value: std::option::Option<String>) {
        self.sys.ismap = value;
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
    /// Get the value of the `decoding` attribute
    pub fn decoding(&self) -> std::option::Option<&str> {
        self.sys.decoding.as_deref()
    }
    /// Set the value of the `decoding` attribute
    pub fn set_decoding(&mut self, value: std::option::Option<String>) {
        self.sys.decoding = value;
    }
    /// Get the value of the `loading` attribute
    pub fn loading(&self) -> std::option::Option<&str> {
        self.sys.loading.as_deref()
    }
    /// Set the value of the `loading` attribute
    pub fn set_loading(&mut self, value: std::option::Option<String>) {
        self.sys.loading = value;
    }
    /// Get the value of the `fetchpriority` attribute
    pub fn fetchpriority(&self) -> std::option::Option<&str> {
        self.sys.fetchpriority.as_deref()
    }
    /// Set the value of the `fetchpriority` attribute
    pub fn set_fetchpriority(&mut self, value: std::option::Option<String>) {
        self.sys.fetchpriority = value;
    }
}
impl crate::categories::FlowContent for Image {}
impl crate::categories::PhrasingContent for Image {}
impl crate::categories::EmbeddedContent for Image {}
impl crate::categories::PalpableContent for Image {}
impl std::convert::Into<html_sys::embedded::Image> for Image {
    fn into(self) -> html_sys::embedded::Image {
        self.sys
    }
}
impl From<html_sys::embedded::Image> for Image {
    fn from(sys: html_sys::embedded::Image) -> Self {
        Self { sys, _children: vec![] }
    }
}
