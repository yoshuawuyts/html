/// The HTML `<th>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
#[doc(alias = "th")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct TableHeader {
    global_attrs: crate::GlobalAttributes,
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
            write!(writer, r#""colspan="{field}""#)?;
        }
        if let Some(field) = self.rowspan.as_ref() {
            write!(writer, r#""rowspan="{field}""#)?;
        }
        if let Some(field) = self.headers.as_ref() {
            write!(writer, r#""headers="{field}""#)?;
        }
        if let Some(field) = self.scope.as_ref() {
            write!(writer, r#""scope="{field}""#)?;
        }
        if let Some(field) = self.abbr.as_ref() {
            write!(writer, r#""abbr="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</th>")?;
        Ok(())
    }
}
impl std::fmt::Display for TableHeader {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for TableHeader {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for TableHeader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
