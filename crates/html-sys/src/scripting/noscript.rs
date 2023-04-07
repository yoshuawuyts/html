/// The HTML `<noscript>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript)
#[doc(alias = "noscript")]
#[non_exhaustive]
pub struct NoScript {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for NoScript {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<noscript")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</noscript>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for NoScript {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for NoScript {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
