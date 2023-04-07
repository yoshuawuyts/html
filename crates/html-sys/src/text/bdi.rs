/// The HTML `<bdi>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
#[doc(alias = "bdi")]
#[non_exhaustive]
pub struct BidirectionalIsolate {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for BidirectionalIsolate {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<bdi")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</bdi>")?;
        Ok(())
    }
}
impl std::ops::Deref for BidirectionalIsolate {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for BidirectionalIsolate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
