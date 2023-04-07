/// The HTML `<map>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
#[doc(alias = "map")]
#[non_exhaustive]
pub struct ImageMap {
    global_attributes: crate::GlobalAttributes,
    /// Name of image map to reference from the usemap attribute
    pub name: std::option::Option<String>,
}
impl crate::RenderElement for ImageMap {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<map")?;
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#""name="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</map>")?;
        Ok(())
    }
}
impl std::ops::Deref for ImageMap {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for ImageMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
