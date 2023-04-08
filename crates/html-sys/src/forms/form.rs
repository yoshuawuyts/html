/// The HTML `<form>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
#[doc(alias = "form")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Form {
    global_attributes: crate::GlobalAttributes,
    /// Character encodings to use for form submission
    pub accept_charset: std::option::Option<String>,
    /// URL to use for form submission
    pub action: std::option::Option<String>,
    /// Default setting for autofill feature for controls in the form
    pub autocomplete: std::option::Option<String>,
    /// Entry list encoding type to use for form submission
    pub enctype: std::option::Option<String>,
    /// Variant to use for form submission
    pub method: std::option::Option<String>,
    /// Name of form to use in the document.forms API
    pub name: std::option::Option<String>,
    /// Bypass form control validation for form submission
    pub novalidate: std::option::Option<String>,
    /// Navigable for form submission
    pub target: std::option::Option<String>,
}
impl crate::RenderElement for Form {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<form")?;
        if let Some(field) = self.accept_charset.as_ref() {
            write!(writer, r#""accept-charset="{field}""#)?;
        }
        if let Some(field) = self.action.as_ref() {
            write!(writer, r#""action="{field}""#)?;
        }
        if let Some(field) = self.autocomplete.as_ref() {
            write!(writer, r#""autocomplete="{field}""#)?;
        }
        if let Some(field) = self.enctype.as_ref() {
            write!(writer, r#""enctype="{field}""#)?;
        }
        if let Some(field) = self.method.as_ref() {
            write!(writer, r#""method="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#""name="{field}""#)?;
        }
        if let Some(field) = self.novalidate.as_ref() {
            write!(writer, r#""novalidate="{field}""#)?;
        }
        if let Some(field) = self.target.as_ref() {
            write!(writer, r#""target="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</form>")?;
        Ok(())
    }
}
impl std::ops::Deref for Form {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Form {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
