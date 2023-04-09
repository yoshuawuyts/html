/// The HTML `<img>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
#[doc(alias = "img")]
#[non_exhaustive]
pub struct Image {
    sys: html_sys::embedded::Image,
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
        Self { sys }
    }
}
