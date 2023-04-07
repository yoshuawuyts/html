/// The HTML `<abbr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
#[doc(alias = "abbr")]
#[non_exhaustive]
pub struct Abbreviation {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for Abbreviation {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<abbr")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</abbr>")?;
        Ok(())
    }
}
impl std::ops::Deref for Abbreviation {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Abbreviation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
