/// The HTML `<figcaption>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
#[doc(alias = "figcaption")]
#[non_exhaustive]
pub struct FigureCaption {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for FigureCaption {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<figcaption")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</figcaption>")?;
        Ok(())
    }
}
impl std::ops::Deref for FigureCaption {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for FigureCaption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
