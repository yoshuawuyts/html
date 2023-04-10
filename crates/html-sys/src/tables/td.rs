/// The HTML `<td>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
#[doc(alias = "td")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct TableCell {
    global_attrs: crate::GlobalAttributes,
    /// Number of columns that the cell is to span
    pub colspan: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Number of rows that the cell is to span
    pub rowspan: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The header cells for this cell
    pub headers: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for TableCell {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<td")?;
        if let Some(field) = self.colspan.as_ref() {
            write!(writer, r#" colspan="{field}""#)?;
        }
        if let Some(field) = self.rowspan.as_ref() {
            write!(writer, r#" rowspan="{field}""#)?;
        }
        if let Some(field) = self.headers.as_ref() {
            write!(writer, r#" headers="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</td>")?;
        Ok(())
    }
}
impl std::fmt::Display for TableCell {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for TableCell {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for TableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
