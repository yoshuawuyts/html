/// The HTML `<td>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
#[doc(alias = "td")]
#[non_exhaustive]
pub struct TableCell {
    global_attributes: crate::GlobalAttributes,
    /// Number of columns that the cell is to span
    pub colspan: std::option::Option<String>,
    /// Number of rows that the cell is to span
    pub rowspan: std::option::Option<String>,
    /// The header cells for this cell
    pub headers: std::option::Option<String>,
}
impl crate::RenderElement for TableCell {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<td")?;
        if let Some(field) = self.colspan.as_ref() {
            write!(writer, r#""colspan="{field}""#)?;
        }
        if let Some(field) = self.rowspan.as_ref() {
            write!(writer, r#""rowspan="{field}""#)?;
        }
        if let Some(field) = self.headers.as_ref() {
            write!(writer, r#""headers="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</td>")?;
        Ok(())
    }
}
impl std::ops::Deref for TableCell {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for TableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
