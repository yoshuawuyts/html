/// The HTML `<picture>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture)
#[doc(alias = "picture")]
#[non_exhaustive]
pub struct Picture {
    sys: html_sys::embedded::Picture,
}
impl crate::categories::FlowContent for Picture {}
impl crate::categories::PhrasingContent for Picture {}
impl crate::categories::EmbeddedContent for Picture {}
impl crate::categories::PalpableContent for Picture {}
impl std::convert::Into<html_sys::embedded::Picture> for Picture {
    fn into(self) -> html_sys::embedded::Picture {
        self.sys
    }
}
impl From<html_sys::embedded::Picture> for Picture {
    fn from(sys: html_sys::embedded::Picture) -> Self {
        Self { sys }
    }
}
