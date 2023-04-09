/// The HTML `<td>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
#[doc(alias = "td")]
#[non_exhaustive]
pub struct TableCell<T: crate::categories::FlowContent> {
    sys: html_sys::tables::TableCell,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::tables::TableCell>
for TableCell<T> {
    fn into(self) -> html_sys::tables::TableCell {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::tables::TableCell>
for TableCell<T> {
    fn from(sys: html_sys::tables::TableCell) -> Self {
        Self { sys, _children: vec![] }
    }
}
