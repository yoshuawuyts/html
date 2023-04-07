/// The HTML `<slot>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
#[doc(alias = "slot")]
#[non_exhaustive]
pub struct Slot {
    global_attributes: crate::GlobalAttributes,
    /// Name of shadow tree slot
    pub name: std::option::Option<String>,
}
impl crate::RenderElement for Slot {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<slot")?;
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#""name="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</slot>")?;
        Ok(())
    }
}
impl std::ops::Deref for Slot {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Slot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
