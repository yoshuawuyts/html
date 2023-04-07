/// The HTML `<dl>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl)
#[doc(alias = "dl")]
#[non_exhaustive]
pub struct DescriptionList {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for DescriptionList {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<dl")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</dl>")?;
        Ok(())
    }
}
impl std::ops::Deref for DescriptionList {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for DescriptionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
