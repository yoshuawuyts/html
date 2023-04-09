/// The HTML `<embed>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
#[doc(alias = "embed")]
#[non_exhaustive]
pub struct Embed {
    sys: html_sys::embedded::Embed,
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
        Self { sys }
    }
}
