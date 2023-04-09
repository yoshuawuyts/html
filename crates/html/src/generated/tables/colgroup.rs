/// The HTML `<colgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
#[doc(alias = "colgroup")]
#[non_exhaustive]
pub struct TableColumnGroup {
    sys: html_sys::tables::TableColumnGroup,
}
impl std::convert::Into<html_sys::tables::TableColumnGroup> for TableColumnGroup {
    fn into(self) -> html_sys::tables::TableColumnGroup {
        self.sys
    }
}
impl From<html_sys::tables::TableColumnGroup> for TableColumnGroup {
    fn from(sys: html_sys::tables::TableColumnGroup) -> Self {
        Self { sys }
    }
}
