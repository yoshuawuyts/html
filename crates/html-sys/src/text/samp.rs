/// The HTML `<samp>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
#[doc(alias = "samp")]
#[non_exhaustive]
pub struct SampleOutput {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for SampleOutput {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<samp")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</samp>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for SampleOutput {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for SampleOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
