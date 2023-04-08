/// The HTML `<map>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
#[doc(alias = "map")]
#[non_exhaustive]
pub struct ImageMap {
    _sys: html_sys::embedded::ImageMap,
}
impl crate::categories::FlowContent for ImageMap {}
impl crate::categories::PhrasingContent for ImageMap {}
impl crate::categories::PalpableContent for ImageMap {}
