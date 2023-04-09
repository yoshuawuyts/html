/// The HTML `<optgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
#[doc(alias = "optgroup")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct OptionGroup {
    global_attrs: crate::GlobalAttributes,
    /// Whether the form control is disabled
    pub disabled: bool,
    /// User-visible label
    pub label: std::option::Option<String>,
}
impl crate::RenderElement for OptionGroup {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<optgroup")?;
        if self.disabled {
            write!(writer, r#" disabled"#)?;
        }
        if let Some(field) = self.label.as_ref() {
            write!(writer, r#" label="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</optgroup>")?;
        Ok(())
    }
}
impl std::fmt::Display for OptionGroup {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for OptionGroup {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for OptionGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
