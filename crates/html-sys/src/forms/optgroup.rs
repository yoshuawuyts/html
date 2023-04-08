/// The HTML `<optgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
#[doc(alias = "optgroup")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct OptionGroup {
    global_attributes: crate::GlobalAttributes,
    /// Whether the form control is disabled
    pub disabled: std::option::Option<String>,
    /// User-visible label
    pub label: std::option::Option<String>,
}
impl crate::RenderElement for OptionGroup {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<optgroup")?;
        if let Some(field) = self.disabled.as_ref() {
            write!(writer, r#""disabled="{field}""#)?;
        }
        if let Some(field) = self.label.as_ref() {
            write!(writer, r#""label="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</optgroup>")?;
        Ok(())
    }
}
impl std::ops::Deref for OptionGroup {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for OptionGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
