/// The HTML `<td>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
#[doc(alias = "td")]
#[non_exhaustive]
pub struct TableCell<T: crate::categories::FlowContent> {
    _sys: html_sys::tables::TableCell,
    _children: Vec<T>,
}
