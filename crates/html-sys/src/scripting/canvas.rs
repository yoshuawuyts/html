/// The HTML `<canvas>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
#[doc(alias = "canvas")]
#[non_exhaustive]
pub struct Canvas {
    global_attributes: crate::GlobalAttributes,
    /// Horizontal dimension
pub width: std::option::Option<String>,
/// Vertical dimension
pub height: std::option::Option<String>,

}

impl crate::RenderElement for Canvas {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<canvas")?;
if let Some(field) = self.width.as_ref() {
    write!(writer, r#""width="{}""#, field)?;
}
if let Some(field) = self.height.as_ref() {
    write!(writer, r#""height="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</canvas>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for Canvas {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
