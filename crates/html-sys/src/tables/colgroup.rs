/// The HTML `<colgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
#[doc(alias = "colgroup")]
#[non_exhaustive]
pub struct TableColumnGroup {
    global_attributes: crate::GlobalAttributes,
    /// Number of columns spanned by the element
pub span: std::option::Option<String>,

}

impl crate::RenderElement for TableColumnGroup {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<colgroup")?;
if let Some(field) = self.span.as_ref() {
    write!(writer, r#""span="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</colgroup>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for TableColumnGroup {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for TableColumnGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
