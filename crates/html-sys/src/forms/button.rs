/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Button {
    global_attributes: crate::GlobalAttributes,
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
    /// Name of the element to use for form submission and in the form.elements API
    pub name: std::option::Option<String>,
    /// Type of button
    pub type_: std::option::Option<String>,
    /// Value to be used for form submission
    pub value: std::option::Option<String>,
}
impl crate::RenderElement for Button {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<button")?;
        if let Some(field) = self.disabled.as_ref() {
            write!(writer, r#""disabled="{field}""#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#""form="{field}""#)?;
        }
        if let Some(field) = self.formaction.as_ref() {
            write!(writer, r#""formaction="{field}""#)?;
        }
        if let Some(field) = self.formenctype.as_ref() {
            write!(writer, r#""formenctype="{field}""#)?;
        }
        if let Some(field) = self.formmethod.as_ref() {
            write!(writer, r#""formmethod="{field}""#)?;
        }
        if let Some(field) = self.formnovalidate.as_ref() {
            write!(writer, r#""formnovalidate="{field}""#)?;
        }
        if let Some(field) = self.formtarget.as_ref() {
            write!(writer, r#""formtarget="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#""name="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#""type="{field}""#)?;
        }
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#""value="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</button>")?;
        Ok(())
    }
}
impl std::ops::Deref for Button {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
