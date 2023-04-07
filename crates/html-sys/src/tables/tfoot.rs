/// The HTML `<tfoot>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
#[doc(alias = "tfoot")]
#[non_exhaustive]
pub struct TableFoot {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for TableFoot {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<tfoot")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</tfoot>")?;
        Ok(())
    }
}
impl std::ops::Deref for TableFoot {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for TableFoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
