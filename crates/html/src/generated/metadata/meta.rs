/// The HTML `<meta>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
#[doc(alias = "meta")]
#[non_exhaustive]
pub struct Meta {
    sys: html_sys::metadata::Meta,
}
impl crate::categories::MetadataContent for Meta {}
impl std::convert::Into<html_sys::metadata::Meta> for Meta {
    fn into(self) -> html_sys::metadata::Meta {
        self.sys
    }
}
impl From<html_sys::metadata::Meta> for Meta {
    fn from(sys: html_sys::metadata::Meta) -> Self {
        Self { sys }
    }
}
