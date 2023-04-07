/// The HTML `<select>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
#[doc(alias = "select")]
#[non_exhaustive]
pub struct Select {
    global_attributes: crate::GlobalAttributes,
    /// Hint for form autofill feature
pub autocomplete: std::option::Option<String>,
/// Whether the form control is disabled
pub disabled: std::option::Option<String>,
/// Associates the element with a form element
pub form: std::option::Option<String>,
/// Whether to allow multiple values
pub multiple: std::option::Option<String>,
/// Name of the element to use for form submission and in the form.elements API
pub name: std::option::Option<String>,
/// Whether the control is required for form submission
pub required: std::option::Option<String>,
/// Size of the control
pub size: std::option::Option<String>,

}

impl crate::RenderElement for Select {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<select")?;
if let Some(field) = self.autocomplete.as_ref() {
    write!(writer, r#""autocomplete="{}""#, field)?;
}
if let Some(field) = self.disabled.as_ref() {
    write!(writer, r#""disabled="{}""#, field)?;
}
if let Some(field) = self.form.as_ref() {
    write!(writer, r#""form="{}""#, field)?;
}
if let Some(field) = self.multiple.as_ref() {
    write!(writer, r#""multiple="{}""#, field)?;
}
if let Some(field) = self.name.as_ref() {
    write!(writer, r#""name="{}""#, field)?;
}
if let Some(field) = self.required.as_ref() {
    write!(writer, r#""required="{}""#, field)?;
}
if let Some(field) = self.size.as_ref() {
    write!(writer, r#""size="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</select>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for Select {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Select {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
