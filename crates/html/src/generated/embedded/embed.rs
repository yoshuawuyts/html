/// The HTML `<embed>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
#[doc(alias = "embed")]
#[non_exhaustive]
pub struct Embed {
    sys: html_sys::embedded::Embed,
    _children: Vec<()>,
}
impl Embed {
    /// Get the value of the `src` attribute
    pub fn src(&self) -> std::option::Option<&str> {
        self.sys.src.as_deref()
    }
    /// Set the value of the `src` attribute
    pub fn set_src(&mut self, value: std::option::Option<String>) {
        self.sys.src = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
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
impl crate::categories::FlowContent for Embed {}
impl crate::categories::PhrasingContent for Embed {}
impl crate::categories::EmbeddedContent for Embed {}
impl crate::categories::InteractiveContent for Embed {}
impl crate::categories::PalpableContent for Embed {}
impl std::convert::Into<html_sys::embedded::Embed> for Embed {
    fn into(self) -> html_sys::embedded::Embed {
        self.sys
    }
}
impl From<html_sys::embedded::Embed> for Embed {
    fn from(sys: html_sys::embedded::Embed) -> Self {
        Self { sys, _children: vec![] }
    }
}
