/// The HTML `<area>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
#[doc(alias = "area")]
#[non_exhaustive]
pub struct ImageMapArea {
    _sys: html_sys::embedded::ImageMapArea,
}
impl crate::categories::FlowContent for ImageMapArea {}
impl crate::categories::PhrasingContent for ImageMapArea {}
