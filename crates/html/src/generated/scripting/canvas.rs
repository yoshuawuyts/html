/// The HTML `<canvas>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
#[doc(alias = "canvas")]
#[non_exhaustive]
pub struct Canvas {
    _sys: html_sys::scripting::Canvas,
}
impl crate::categories::FlowContent for Canvas {}
impl crate::categories::PhrasingContent for Canvas {}
impl crate::categories::EmbeddedContent for Canvas {}
impl crate::categories::PalpableContent for Canvas {}
