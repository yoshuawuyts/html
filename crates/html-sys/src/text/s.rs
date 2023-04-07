/// The HTML `<s>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
#[doc(alias = "s")]
#[non_exhaustive]
pub struct StrikeThrough {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for StrikeThrough {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<s")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</s>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for StrikeThrough {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for StrikeThrough {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
