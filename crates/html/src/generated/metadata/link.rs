/// The HTML `<link>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
#[doc(alias = "link")]
#[non_exhaustive]
pub struct Link {
    sys: html_sys::metadata::Link,
}
impl crate::categories::MetadataContent for Link {}
impl std::convert::Into<html_sys::metadata::Link> for Link {
    fn into(self) -> html_sys::metadata::Link {
        self.sys
    }
}
impl From<html_sys::metadata::Link> for Link {
    fn from(sys: html_sys::metadata::Link) -> Self {
        Self { sys }
    }
}
