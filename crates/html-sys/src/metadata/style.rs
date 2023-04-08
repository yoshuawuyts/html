/// The HTML `<style>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
#[doc(alias = "style")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Style {
    global_attributes: crate::GlobalAttributes,
    /// Applicable media
    pub media: std::option::Option<String>,
    /// Whether the element is potentially render-blocking
    pub blocking: std::option::Option<String>,
}
impl crate::RenderElement for Style {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<style")?;
        if let Some(field) = self.media.as_ref() {
            write!(writer, r#""media="{field}""#)?;
        }
        if let Some(field) = self.blocking.as_ref() {
            write!(writer, r#""blocking="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</style>")?;
        Ok(())
    }
}
impl std::ops::Deref for Style {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
