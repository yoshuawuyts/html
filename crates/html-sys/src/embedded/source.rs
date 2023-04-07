/// The HTML `<source>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
#[doc(alias = "source")]
#[non_exhaustive]
pub struct MediaSource {
    global_attributes: crate::GlobalAttributes,
    /// Type of embedded resource
pub type_: std::option::Option<String>,

}

impl crate::RenderElement for MediaSource {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<source")?;
if let Some(field) = self.type_.as_ref() {
    write!(writer, r#""type="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        
        Ok(())
    }
}
impl std::ops::Deref for MediaSource {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for MediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
