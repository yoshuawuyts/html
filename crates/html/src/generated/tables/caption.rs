/// The HTML `<caption>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/caption)
#[doc(alias = "caption")]
#[non_exhaustive]
pub struct Caption {
    sys: html_sys::tables::Caption,
}
impl std::convert::Into<html_sys::tables::Caption> for Caption {
    fn into(self) -> html_sys::tables::Caption {
        self.sys
    }
}
impl From<html_sys::tables::Caption> for Caption {
    fn from(sys: html_sys::tables::Caption) -> Self {
        Self { sys }
    }
}
