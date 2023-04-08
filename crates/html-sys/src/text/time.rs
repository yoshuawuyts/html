/// The HTML `<time>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
#[doc(alias = "time")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Time {
    global_attributes: crate::GlobalAttributes,
    /// Machine-readable value
    pub datetime: std::option::Option<String>,
}
impl crate::RenderElement for Time {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<time")?;
        if let Some(field) = self.datetime.as_ref() {
            write!(writer, r#""datetime="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</time>")?;
        Ok(())
    }
}
impl std::ops::Deref for Time {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
