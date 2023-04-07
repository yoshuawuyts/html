/// The HTML `<ruby>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby)
#[doc(alias = "ruby")]
#[non_exhaustive]
pub struct RubyAnnotation {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for RubyAnnotation {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<ruby")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</ruby>")?;
        Ok(())
    }
}
impl std::ops::Deref for RubyAnnotation {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for RubyAnnotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
