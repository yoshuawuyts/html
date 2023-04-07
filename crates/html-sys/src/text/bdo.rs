/// The HTML `<bdo>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
#[doc(alias = "bdo")]
#[non_exhaustive]
pub struct BidirectionalTextOverride {
    global_attributes: crate::GlobalAttributes,
}
impl crate::RenderElement for BidirectionalTextOverride {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<bdo")?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</bdo>")?;
        Ok(())
    }
}
impl std::ops::Deref for BidirectionalTextOverride {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for BidirectionalTextOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
