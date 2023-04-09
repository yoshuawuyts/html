/// The HTML `<style>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
#[doc(alias = "style")]
#[non_exhaustive]
pub struct Style {
    sys: html_sys::metadata::Style,
}
impl crate::categories::MetadataContent for Style {}
impl std::convert::Into<html_sys::metadata::Style> for Style {
    fn into(self) -> html_sys::metadata::Style {
        self.sys
    }
}
impl From<html_sys::metadata::Style> for Style {
    fn from(sys: html_sys::metadata::Style) -> Self {
        Self { sys }
    }
}
