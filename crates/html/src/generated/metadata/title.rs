/// The HTML `<title>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
#[doc(alias = "title")]
#[non_exhaustive]
pub struct Title {
    sys: html_sys::metadata::Title,
}
impl crate::categories::MetadataContent for Title {}
impl std::convert::Into<html_sys::metadata::Title> for Title {
    fn into(self) -> html_sys::metadata::Title {
        self.sys
    }
}
impl From<html_sys::metadata::Title> for Title {
    fn from(sys: html_sys::metadata::Title) -> Self {
        Self { sys }
    }
}
