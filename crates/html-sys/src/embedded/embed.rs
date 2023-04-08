/// The HTML `<embed>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
#[doc(alias = "embed")]
#[non_exhaustive]
pub struct Embed {
    global_attributes: crate::GlobalAttributes,
    /// Address of the resource
    pub src: std::option::Option<String>,
    /// Type of embedded resource
    pub type_: std::option::Option<String>,
    /// Horizontal dimension
    pub width: std::option::Option<String>,
    /// Vertical dimension
    pub height: std::option::Option<String>,
}
impl crate::RenderElement for Embed {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<embed")?;
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#""src="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#""type="{field}""#)?;
        }
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#""width="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#""height="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::ops::Deref for Embed {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Embed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
