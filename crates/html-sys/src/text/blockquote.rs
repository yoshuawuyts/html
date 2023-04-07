/// The HTML `<blockquote>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
#[doc(alias = "blockquote")]
#[non_exhaustive]
pub struct BlockQuote {
    global_attributes: crate::GlobalAttributes,
    /// Link to the source of the quotation or more information about the edit
    pub cite: std::option::Option<String>,
}
impl crate::RenderElement for BlockQuote {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<blockquote")?;
        if let Some(field) = self.cite.as_ref() {
            write!(writer, r#""cite="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</blockquote>")?;
        Ok(())
    }
}
impl std::ops::Deref for BlockQuote {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for BlockQuote {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
