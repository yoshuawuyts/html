/// The HTML `<progress>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
#[doc(alias = "progress")]
#[non_exhaustive]
pub struct Progress {
    global_attributes: crate::GlobalAttributes,
    /// Current value of the element
    pub value: std::option::Option<String>,
    /// Upper bound of range
    pub max: std::option::Option<String>,
}
impl crate::RenderElement for Progress {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<progress")?;
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#""value="{}""#, field)?;
        }
        if let Some(field) = self.max.as_ref() {
            write!(writer, r#""max="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</progress>")?;
        Ok(())
    }
}
impl std::ops::Deref for Progress {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Progress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
