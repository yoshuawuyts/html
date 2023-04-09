/// The HTML `<table>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
#[doc(alias = "table")]
#[non_exhaustive]
pub struct Table {
    sys: html_sys::tables::Table,
    _children: Vec<()>,
}
impl crate::HtmlElement for Table {}
impl crate::categories::FlowContent for Table {}
impl crate::categories::PalpableContent for Table {}
impl std::convert::Into<html_sys::tables::Table> for Table {
    fn into(self) -> html_sys::tables::Table {
        self.sys
    }
}
impl From<html_sys::tables::Table> for Table {
    fn from(sys: html_sys::tables::Table) -> Self {
        Self { sys, _children: vec![] }
    }
}
