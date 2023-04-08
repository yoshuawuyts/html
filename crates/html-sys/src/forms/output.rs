/// The HTML `<output>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
#[doc(alias = "output")]
#[non_exhaustive]
pub struct Output {
    global_attributes: crate::GlobalAttributes,
    /// Specifies controls from which the output was calculated
    pub for_: std::option::Option<String>,
    /// Associates the element with a form element
    pub form: std::option::Option<String>,
    /// Name of the element to use in the form.elements API.
    pub name: std::option::Option<String>,
}
impl crate::RenderElement for Output {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<output")?;
        if let Some(field) = self.for_.as_ref() {
            write!(writer, r#""for="{field}""#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#""form="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#""name="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</output>")?;
        Ok(())
    }
}
impl std::ops::Deref for Output {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Output {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
