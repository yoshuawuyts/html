/// The HTML `<th>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
#[doc(alias = "th")]
#[non_exhaustive]
pub struct TableHeader {
    global_attributes: crate::GlobalAttributes,
    /// Number of columns that the cell is to span
    pub colspan: std::option::Option<String>,
    /// Number of rows that the cell is to span
    pub rowspan: std::option::Option<String>,
    /// The header cells for this cell
    pub headers: std::option::Option<String>,
    /// Specifies which cells the header cell applies to
    pub scope: std::option::Option<String>,
    /// Alternative label to use for the header cell when referencing the cell in other contexts
    pub abbr: std::option::Option<String>,
}
impl crate::RenderElement for TableHeader {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<th")?;
        if let Some(field) = self.colspan.as_ref() {
            write!(writer, r#""colspan="{}""#, field)?;
        }
        if let Some(field) = self.rowspan.as_ref() {
            write!(writer, r#""rowspan="{}""#, field)?;
        }
        if let Some(field) = self.headers.as_ref() {
            write!(writer, r#""headers="{}""#, field)?;
        }
        if let Some(field) = self.scope.as_ref() {
            write!(writer, r#""scope="{}""#, field)?;
        }
        if let Some(field) = self.abbr.as_ref() {
            write!(writer, r#""abbr="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</th>")?;
        Ok(())
    }
}
impl std::ops::Deref for TableHeader {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for TableHeader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
