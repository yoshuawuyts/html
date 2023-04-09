/// The HTML `<area>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
#[doc(alias = "area")]
#[non_exhaustive]
pub struct ImageMapArea {
    sys: html_sys::embedded::ImageMapArea,
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
        Self { sys }
    }
}
