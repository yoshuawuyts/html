/// The HTML `<textarea>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct TextArea {
    global_attrs: crate::GlobalAttributes,
    /// Hint for form autofill feature
    pub autocomplete: std::option::Option<String>,
    /// Maximum number of characters per line
    pub cols: std::option::Option<i64>,
    /// Name of form control to use for sending the element's directionality in form submission
    pub dir_name: std::option::Option<String>,
    /// Whether the form control is disabled
    pub disabled: std::option::Option<bool>,
    /// Associates the element with a form element
    pub form: std::option::Option<String>,
    /// Maximum length of value
    pub max_length: std::option::Option<i64>,
    /// Minimum length of value
    pub min_length: std::option::Option<i64>,
    /// Name of the element to use for form submission and in the form.elements API
    pub name: std::option::Option<String>,
    /// User-visible label to be placed within the form control
    pub placeholder: std::option::Option<String>,
    /// Whether to allow the value to be edited by the user
    pub read_only: std::option::Option<bool>,
    /// Whether the control is required for form submission
    pub required: std::option::Option<bool>,
    /// Number of lines to show
    pub rows: std::option::Option<i64>,
    /// How the value of the form control is to be wrapped for form submission
    pub wrap: std::option::Option<String>,
}
impl crate::RenderElement for TextArea {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<textarea")?;
        if let Some(field) = self.autocomplete.as_ref() {
            write!(writer, r#" autocomplete="{field}""#)?;
        }
        if let Some(field) = self.cols.as_ref() {
            write!(writer, r#" cols="{field}""#)?;
        }
        if let Some(field) = self.dir_name.as_ref() {
            write!(writer, r#" dirname="{field}""#)?;
        }
        if let Some(field) = self.disabled.as_ref() {
            if *field {
                write!(writer, r#" disabled"#)?;
            }
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#" form="{field}""#)?;
        }
        if let Some(field) = self.max_length.as_ref() {
            write!(writer, r#" maxlength="{field}""#)?;
        }
        if let Some(field) = self.min_length.as_ref() {
            write!(writer, r#" minlength="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        if let Some(field) = self.placeholder.as_ref() {
            write!(writer, r#" placeholder="{field}""#)?;
        }
        if let Some(field) = self.read_only.as_ref() {
            if *field {
                write!(writer, r#" readonly"#)?;
            }
        }
        if let Some(field) = self.required.as_ref() {
            if *field {
                write!(writer, r#" required"#)?;
            }
        }
        if let Some(field) = self.rows.as_ref() {
            write!(writer, r#" rows="{field}""#)?;
        }
        if let Some(field) = self.wrap.as_ref() {
            write!(writer, r#" wrap="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</textarea>")?;
        Ok(())
    }
}
impl std::fmt::Display for TextArea {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for TextArea {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for TextArea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
