/// The HTML `<title>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
#[doc(alias = "title")]
#[non_exhaustive]
pub struct Title {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for Title {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<title")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</title>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for Title {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Title {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
