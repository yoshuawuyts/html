/// The HTML `<del>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
#[doc(alias = "del")]
#[non_exhaustive]
pub struct DeletedText {
    global_attributes: crate::GlobalAttributes,
    /// Link to the source of the quotation or more information about the edit
pub cite: std::option::Option<String>,
/// Date and (optionally) time of the change
pub datetime: std::option::Option<String>,

}

impl crate::RenderElement for DeletedText {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<del")?;
if let Some(field) = self.cite.as_ref() {
    write!(writer, r#""cite="{}""#, field)?;
}
if let Some(field) = self.datetime.as_ref() {
    write!(writer, r#""datetime="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</del>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for DeletedText {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for DeletedText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
