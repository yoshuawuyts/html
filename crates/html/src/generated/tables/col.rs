/// The HTML `<col>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
#[doc(alias = "col")]
#[non_exhaustive]
pub struct TableColumn {
    sys: html_sys::tables::TableColumn,
}
impl std::convert::Into<html_sys::tables::TableColumn> for TableColumn {
    fn into(self) -> html_sys::tables::TableColumn {
        self.sys
    }
}
impl From<html_sys::tables::TableColumn> for TableColumn {
    fn from(sys: html_sys::tables::TableColumn) -> Self {
        Self { sys }
    }
}
