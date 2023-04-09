/// The HTML `<tr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr)
#[doc(alias = "tr")]
#[non_exhaustive]
pub struct TableRow {
    sys: html_sys::tables::TableRow,
}
impl crate::HtmlElement for TableRow {}
impl std::convert::Into<html_sys::tables::TableRow> for TableRow {
    fn into(self) -> html_sys::tables::TableRow {
        self.sys
    }
}
impl From<html_sys::tables::TableRow> for TableRow {
    fn from(sys: html_sys::tables::TableRow) -> Self {
        Self { sys }
    }
}
