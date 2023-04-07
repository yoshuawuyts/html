/// The HTML `<input>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[doc(alias = "input")]
#[non_exhaustive]
pub struct Input {
    global_attributes: crate::GlobalAttributes,
    /// Hint for expected file type in file upload controls
pub accept: std::option::Option<String>,
/// Replacement text for use when images are not available
pub alt: std::option::Option<String>,
/// Hint for form autofill feature
pub autocomplete: std::option::Option<String>,
/// Whether the control is checked
pub checked: std::option::Option<String>,
/// Name of form control to use for sending the element's directionality in form submission
pub dirname: std::option::Option<String>,
/// Whether the form control is disabled
pub disabled: std::option::Option<String>,
/// Associates the element with a form element
pub form: std::option::Option<String>,
/// URL to use for form submission
pub formaction: std::option::Option<String>,
/// Entry list encoding type to use for form submission
pub formenctype: std::option::Option<String>,
/// Variant to use for form submission
pub formmethod: std::option::Option<String>,
/// Bypass form control validation for form submission
pub formnovalidate: std::option::Option<String>,
/// Navigable for form submission
pub formtarget: std::option::Option<String>,
/// Vertical dimension
pub height: std::option::Option<String>,
/// List of autocomplete options
pub list: std::option::Option<String>,
/// Maximum value
pub max: std::option::Option<String>,
/// Maximum length of value
pub maxlength: std::option::Option<String>,
/// Minimum value
pub min: std::option::Option<String>,
/// Minimum length of value
pub minlength: std::option::Option<String>,
/// Whether to allow multiple values
pub multiple: std::option::Option<String>,
/// Name of the element to use for form submission and in the form.elements API
pub name: std::option::Option<String>,
/// Pattern to be matched by the form control's value
pub pattern: std::option::Option<String>,
/// User-visible label to be placed within the form control
pub placeholder: std::option::Option<String>,
/// Whether to allow the value to be edited by the user
pub readonly: std::option::Option<String>,
/// Whether the control is required for form submission
pub required: std::option::Option<String>,
/// Size of the control
pub size: std::option::Option<String>,
/// Address of the resource
pub src: std::option::Option<String>,
/// Granularity to be matched by the form control's value
pub step: std::option::Option<String>,
/// Type of form control
pub type_: std::option::Option<String>,
/// Value of the form control
pub value: std::option::Option<String>,
/// Horizontal dimension
pub width: std::option::Option<String>,

}

impl crate::RenderElement for Input {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<input")?;
if let Some(field) = self.accept.as_ref() {
    write!(writer, r#""accept="{}""#, field)?;
}
if let Some(field) = self.alt.as_ref() {
    write!(writer, r#""alt="{}""#, field)?;
}
if let Some(field) = self.autocomplete.as_ref() {
    write!(writer, r#""autocomplete="{}""#, field)?;
}
if let Some(field) = self.checked.as_ref() {
    write!(writer, r#""checked="{}""#, field)?;
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
if let Some(field) = self.formaction.as_ref() {
    write!(writer, r#""formaction="{}""#, field)?;
}
if let Some(field) = self.formenctype.as_ref() {
    write!(writer, r#""formenctype="{}""#, field)?;
}
if let Some(field) = self.formmethod.as_ref() {
    write!(writer, r#""formmethod="{}""#, field)?;
}
if let Some(field) = self.formnovalidate.as_ref() {
    write!(writer, r#""formnovalidate="{}""#, field)?;
}
if let Some(field) = self.formtarget.as_ref() {
    write!(writer, r#""formtarget="{}""#, field)?;
}
if let Some(field) = self.height.as_ref() {
    write!(writer, r#""height="{}""#, field)?;
}
if let Some(field) = self.list.as_ref() {
    write!(writer, r#""list="{}""#, field)?;
}
if let Some(field) = self.max.as_ref() {
    write!(writer, r#""max="{}""#, field)?;
}
if let Some(field) = self.maxlength.as_ref() {
    write!(writer, r#""maxlength="{}""#, field)?;
}
if let Some(field) = self.min.as_ref() {
    write!(writer, r#""min="{}""#, field)?;
}
if let Some(field) = self.minlength.as_ref() {
    write!(writer, r#""minlength="{}""#, field)?;
}
if let Some(field) = self.multiple.as_ref() {
    write!(writer, r#""multiple="{}""#, field)?;
}
if let Some(field) = self.name.as_ref() {
    write!(writer, r#""name="{}""#, field)?;
}
if let Some(field) = self.pattern.as_ref() {
    write!(writer, r#""pattern="{}""#, field)?;
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
if let Some(field) = self.size.as_ref() {
    write!(writer, r#""size="{}""#, field)?;
}
if let Some(field) = self.src.as_ref() {
    write!(writer, r#""src="{}""#, field)?;
}
if let Some(field) = self.step.as_ref() {
    write!(writer, r#""step="{}""#, field)?;
}
if let Some(field) = self.type_.as_ref() {
    write!(writer, r#""type="{}""#, field)?;
}
if let Some(field) = self.value.as_ref() {
    write!(writer, r#""value="{}""#, field)?;
}
if let Some(field) = self.width.as_ref() {
    write!(writer, r#""width="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        
        Ok(())
    }
}
impl std::ops::Deref for Input {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
