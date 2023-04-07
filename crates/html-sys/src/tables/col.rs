/// The HTML `<col>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
#[doc(alias = "col")]
#[non_exhaustive]
pub struct TableColumn {
    global_attributes: crate::GlobalAttributes,
    /// Number of columns spanned by the element
pub span: std::option::Option<String>,

}

impl crate::RenderElement for TableColumn {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<col")?;
if let Some(field) = self.span.as_ref() {
    write!(writer, r#""span="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        
        Ok(())
    }
}
impl std::ops::Deref for TableColumn {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for TableColumn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
