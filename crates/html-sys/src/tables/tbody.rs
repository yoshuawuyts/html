/// The HTML `<tbody>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
#[doc(alias = "tbody")]
#[non_exhaustive]
pub struct TableBody {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for TableBody {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<tbody")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</tbody>")?;
        Ok(())
    }
}
impl std::ops::Deref for TableBody {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for TableBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
