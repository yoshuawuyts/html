/// The HTML `<canvas>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
#[doc(alias = "canvas")]
#[non_exhaustive]
pub struct Canvas {
    sys: html_sys::scripting::Canvas,
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
