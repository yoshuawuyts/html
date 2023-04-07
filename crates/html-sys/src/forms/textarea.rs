/// The HTML `<textarea>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[non_exhaustive]
pub struct TextArea {
    global_attributes: crate::GlobalAttributes,
    /// Hint for form autofill feature
pub autocomplete: std::option::Option<String>,
/// Maximum number of characters per line
pub cols: std::option::Option<String>,
/// Name of form control to use for sending the element's directionality in form submission
pub dirname: std::option::Option<String>,
/// Whether the form control is disabled
pub disabled: std::option::Option<String>,
/// Associates the element with a form element
pub form: std::option::Option<String>,
/// Maximum length of value
pub maxlength: std::option::Option<String>,
/// Minimum length of value
pub minlength: std::option::Option<String>,
/// Name of the element to use for form submission and in the form.elements API
pub name: std::option::Option<String>,
/// User-visible label to be placed within the form control
pub placeholder: std::option::Option<String>,
/// Whether to allow the value to be edited by the user
pub readonly: std::option::Option<String>,
/// Whether the control is required for form submission
pub required: std::option::Option<String>,
/// Number of lines to show
pub rows: std::option::Option<String>,
/// How the value of the form control is to be wrapped for form submission
pub wrap: std::option::Option<String>,

}

impl crate::RenderElement for TextArea {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<textarea")?;
if let Some(field) = self.autocomplete.as_ref() {
    write!(writer, r#""autocomplete="{}""#, field)?;
}
if let Some(field) = self.cols.as_ref() {
    write!(writer, r#""cols="{}""#, field)?;
}
if let Some(field) = self.dirname.as_ref() {
    write!(writer, r#""dirname="{}""#, field)?;
}
if let Some(field) = self.disabled.as_ref() {
    write!(writer, r#""disabled="{}""#, field)?;
}
if let Some(field) = self.form.as_ref() {
    write!(writer, r#""form="{}""#, field)?;
}
if let Some(field) = self.maxlength.as_ref() {
    write!(writer, r#""maxlength="{}""#, field)?;
}
if let Some(field) = self.minlength.as_ref() {
    write!(writer, r#""minlength="{}""#, field)?;
}
if let Some(field) = self.name.as_ref() {
    write!(writer, r#""name="{}""#, field)?;
}
if let Some(field) = self.placeholder.as_ref() {
    write!(writer, r#""placeholder="{}""#, field)?;
}
if let Some(field) = self.readonly.as_ref() {
    write!(writer, r#""readonly="{}""#, field)?;
}
if let Some(field) = self.required.as_ref() {
    write!(writer, r#""required="{}""#, field)?;
}
if let Some(field) = self.rows.as_ref() {
    write!(writer, r#""rows="{}""#, field)?;
}
if let Some(field) = self.wrap.as_ref() {
    write!(writer, r#""wrap="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</textarea>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for TextArea {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for TextArea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
