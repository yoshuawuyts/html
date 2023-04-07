/// The HTML `<dd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
#[doc(alias = "dd")]
#[non_exhaustive]
pub struct DescriptionDetails {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for DescriptionDetails {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<dd")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</dd>")?;
        Ok(())
    }
}
impl std::ops::Deref for DescriptionDetails {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for DescriptionDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
