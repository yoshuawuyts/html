/// The HTML `<hgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
#[doc(alias = "hgroup")]
#[non_exhaustive]
pub struct HeadingGroup {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for HeadingGroup {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<hgroup")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</hgroup>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for HeadingGroup {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for HeadingGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
