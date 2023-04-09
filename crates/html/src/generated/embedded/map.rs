/// The HTML `<map>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
#[doc(alias = "map")]
#[non_exhaustive]
pub struct ImageMap {
    sys: html_sys::embedded::ImageMap,
}
impl crate::categories::FlowContent for ImageMap {}
impl crate::categories::PhrasingContent for ImageMap {}
impl crate::categories::PalpableContent for ImageMap {}
impl std::convert::Into<html_sys::embedded::ImageMap> for ImageMap {
    fn into(self) -> html_sys::embedded::ImageMap {
        self.sys
    }
}
impl From<html_sys::embedded::ImageMap> for ImageMap {
    fn from(sys: html_sys::embedded::ImageMap) -> Self {
        Self { sys }
    }
}
