/// The HTML `<canvas>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
#[doc(alias = "canvas")]
#[non_exhaustive]
pub struct Canvas {
    sys: html_sys::scripting::Canvas,
}
impl Canvas {
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
impl crate::categories::FlowContent for Canvas {}
impl crate::categories::PhrasingContent for Canvas {}
impl crate::categories::EmbeddedContent for Canvas {}
impl crate::categories::PalpableContent for Canvas {}
impl std::convert::Into<html_sys::scripting::Canvas> for Canvas {
    fn into(self) -> html_sys::scripting::Canvas {
        self.sys
    }
}
impl From<html_sys::scripting::Canvas> for Canvas {
    fn from(sys: html_sys::scripting::Canvas) -> Self {
        Self { sys }
    }
}
