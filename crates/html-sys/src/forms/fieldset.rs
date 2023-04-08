/// The HTML `<fieldset>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
#[doc(alias = "fieldset")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Fieldset {
    global_attrs: crate::GlobalAttributes,
    /// Whether the descendant form controls, except any inside legend, are disabled
    pub disabled: std::option::Option<String>,
    /// Associates the element with a form element
    pub form: std::option::Option<String>,
    /// Name of the element to use in the form.elements API.
    pub name: std::option::Option<String>,
}
impl crate::RenderElement for Fieldset {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<fieldset")?;
        if let Some(field) = self.disabled.as_ref() {
            write!(writer, r#""disabled="{field}""#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#""form="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#""name="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</fieldset>")?;
        Ok(())
    }
}
impl std::fmt::Display for Fieldset {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Fieldset {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Fieldset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
