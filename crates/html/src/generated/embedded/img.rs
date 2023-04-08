/// The HTML `<img>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
#[doc(alias = "img")]
#[non_exhaustive]
pub struct Image {
    _sys: html_sys::embedded::Image,
}
impl crate::categories::FlowContent for Image {}
impl crate::categories::PhrasingContent for Image {}
impl crate::categories::EmbeddedContent for Image {}
impl crate::categories::PalpableContent for Image {}
