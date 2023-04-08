/// The HTML `<style>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
#[doc(alias = "style")]
#[non_exhaustive]
pub struct Style {
    _sys: html_sys::metadata::Style,
}
impl crate::categories::MetadataContent for Style {}
