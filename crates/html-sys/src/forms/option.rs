/// The HTML `<option>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option)
#[doc(alias = "option")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Option {
    global_attrs: crate::GlobalAttributes,
    /// Whether the form control is disabled
    pub disabled: std::option::Option<String>,
    /// User-visible label
    pub label: std::option::Option<String>,
    /// Whether the option is selected by default
    pub selected: std::option::Option<String>,
    /// Value to be used for form submission
    pub value: std::option::Option<String>,
}
impl crate::RenderElement for Option {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<option")?;
        if let Some(field) = self.disabled.as_ref() {
            write!(writer, r#" disabled="{field}""#)?;
        }
        if let Some(field) = self.label.as_ref() {
            write!(writer, r#" label="{field}""#)?;
        }
        if let Some(field) = self.selected.as_ref() {
            write!(writer, r#" selected="{field}""#)?;
        }
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#" value="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</option>")?;
        Ok(())
    }
}
impl std::fmt::Display for Option {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Option {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Option {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
