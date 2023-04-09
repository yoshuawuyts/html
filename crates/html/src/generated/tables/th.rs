/// The HTML `<th>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
#[doc(alias = "th")]
#[non_exhaustive]
pub struct TableHeader {
    sys: html_sys::tables::TableHeader,
}
impl std::convert::Into<html_sys::tables::TableHeader> for TableHeader {
    fn into(self) -> html_sys::tables::TableHeader {
        self.sys
    }
}
impl From<html_sys::tables::TableHeader> for TableHeader {
    fn from(sys: html_sys::tables::TableHeader) -> Self {
        Self { sys }
    }
}
