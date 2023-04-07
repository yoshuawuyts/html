/// The HTML `<dt>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt)
#[doc(alias = "dt")]
#[non_exhaustive]
pub struct DescriptionTerm {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for DescriptionTerm {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<dt")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</dt>")?;
        Ok(())
    }
}
impl std::ops::Deref for DescriptionTerm {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for DescriptionTerm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
