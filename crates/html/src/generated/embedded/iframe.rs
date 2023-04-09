/// The HTML `<iframe>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
#[doc(alias = "iframe")]
#[non_exhaustive]
pub struct Iframe {
    sys: html_sys::embedded::Iframe,
}
impl crate::categories::FlowContent for Iframe {}
impl crate::categories::PhrasingContent for Iframe {}
impl crate::categories::EmbeddedContent for Iframe {}
impl crate::categories::InteractiveContent for Iframe {}
impl crate::categories::PalpableContent for Iframe {}
impl std::convert::Into<html_sys::embedded::Iframe> for Iframe {
    fn into(self) -> html_sys::embedded::Iframe {
        self.sys
    }
}
impl From<html_sys::embedded::Iframe> for Iframe {
    fn from(sys: html_sys::embedded::Iframe) -> Self {
        Self { sys }
    }
}
