/// The HTML `<area>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
#[doc(alias = "area")]
#[non_exhaustive]
pub struct ImageMapArea {
    sys: html_sys::embedded::ImageMapArea,
    _children: Vec<()>,
}
impl ImageMapArea {
    /// Get the value of the `alt` attribute
    pub fn alt(&self) -> std::option::Option<&str> {
        self.sys.alt.as_deref()
    }
    /// Set the value of the `alt` attribute
    pub fn set_alt(&mut self, value: std::option::Option<String>) {
        self.sys.alt = value;
    }
    /// Get the value of the `coords` attribute
    pub fn coords(&self) -> std::option::Option<&str> {
        self.sys.coords.as_deref()
    }
    /// Set the value of the `coords` attribute
    pub fn set_coords(&mut self, value: std::option::Option<String>) {
        self.sys.coords = value;
    }
    /// Get the value of the `shape` attribute
    pub fn shape(&self) -> std::option::Option<&str> {
        self.sys.shape.as_deref()
    }
    /// Set the value of the `shape` attribute
    pub fn set_shape(&mut self, value: std::option::Option<String>) {
        self.sys.shape = value;
    }
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
    /// Get the value of the `referrerpolicy` attribute
    pub fn referrerpolicy(&self) -> std::option::Option<&str> {
        self.sys.referrerpolicy.as_deref()
    }
    /// Set the value of the `referrerpolicy` attribute
    pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
        self.sys.referrerpolicy = value;
    }
}
impl crate::categories::FlowContent for ImageMapArea {}
impl crate::categories::PhrasingContent for ImageMapArea {}
impl std::convert::Into<html_sys::embedded::ImageMapArea> for ImageMapArea {
    fn into(self) -> html_sys::embedded::ImageMapArea {
        self.sys
    }
}
impl From<html_sys::embedded::ImageMapArea> for ImageMapArea {
    fn from(sys: html_sys::embedded::ImageMapArea) -> Self {
        Self { sys, _children: vec![] }
    }
}
