/// The HTML `<label>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
#[doc(alias = "label")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Label {
    global_attributes: crate::GlobalAttributes,
    /// Associate the label with form control
    pub for_: std::option::Option<String>,
}
impl crate::RenderElement for Label {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<label")?;
        if let Some(field) = self.for_.as_ref() {
            write!(writer, r#""for="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</label>")?;
        Ok(())
    }
}
impl std::ops::Deref for Label {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
