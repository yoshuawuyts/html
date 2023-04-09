/// The HTML `<base>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
#[doc(alias = "base")]
#[non_exhaustive]
pub struct Base {
    sys: html_sys::metadata::Base,
}
impl crate::categories::MetadataContent for Base {}
impl std::convert::Into<html_sys::metadata::Base> for Base {
    fn into(self) -> html_sys::metadata::Base {
        self.sys
    }
}
impl From<html_sys::metadata::Base> for Base {
    fn from(sys: html_sys::metadata::Base) -> Self {
        Self { sys }
    }
}
