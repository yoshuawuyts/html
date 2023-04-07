/// The HTML `<kbd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
#[doc(alias = "kbd")]
#[non_exhaustive]
pub struct KeyboardInput {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for KeyboardInput {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<kbd")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</kbd>")?;
        Ok(())
    }
}
impl std::ops::Deref for KeyboardInput {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for KeyboardInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
