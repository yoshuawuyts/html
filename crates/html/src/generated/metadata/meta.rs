/// The HTML `<meta>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
#[doc(alias = "meta")]
#[non_exhaustive]
pub struct Meta {
    _sys: html_sys::metadata::Meta,
}
impl crate::categories::MetadataContent for Meta {}
