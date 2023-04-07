/// The HTML `<fieldset>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
#[doc(alias = "fieldset")]
#[non_exhaustive]
pub struct Fieldset {
    global_attributes: crate::GlobalAttributes,
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
    write!(writer, r#""disabled="{}""#, field)?;
}
if let Some(field) = self.form.as_ref() {
    write!(writer, r#""form="{}""#, field)?;
}
if let Some(field) = self.name.as_ref() {
    write!(writer, r#""name="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</fieldset>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for Fieldset {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Fieldset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
