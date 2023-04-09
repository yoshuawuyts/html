/// The HTML `<noscript>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript)
#[doc(alias = "noscript")]
#[non_exhaustive]
pub struct NoScript {
    sys: html_sys::scripting::NoScript,
}
impl crate::categories::MetadataContent for NoScript {}
impl crate::categories::FlowContent for NoScript {}
impl crate::categories::PhrasingContent for NoScript {}
impl std::convert::Into<html_sys::scripting::NoScript> for NoScript {
    fn into(self) -> html_sys::scripting::NoScript {
        self.sys
    }
}
impl From<html_sys::scripting::NoScript> for NoScript {
    fn from(sys: html_sys::scripting::NoScript) -> Self {
        Self { sys }
    }
}
