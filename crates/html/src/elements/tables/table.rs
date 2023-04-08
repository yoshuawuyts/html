/// The HTML `<table>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
#[doc(alias = "table")]
#[non_exhaustive]
pub struct Table {
    sys: html_sys::tables::Table,
}
