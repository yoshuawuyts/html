/// The HTML `<input>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[doc(alias = "input")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Input {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Hint for expected file type in file upload controls
    pub accept: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Replacement text for use when images are not available
    pub alt: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hint for form autofill feature
    pub autocomplete: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the control is checked
    pub checked: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Name of form control to use for sending the element's directionality in form submission
    pub dirname: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the form control is disabled
    pub disabled: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Associates the element with a form element
    pub form: std::option::Option<std::borrow::Cow<'static, str>>,
    /// URL to use for form submission
    pub formaction: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Entry list encoding type to use for form submission
    pub formenctype: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Variant to use for form submission
    pub formmethod: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Bypass form control validation for form submission
    pub formnovalidate: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Navigable for form submission
    pub formtarget: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Vertical dimension
    pub height: std::option::Option<std::borrow::Cow<'static, str>>,
    /// List of autocomplete options
    pub list: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Maximum value
    pub max: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Maximum length of value
    pub maxlength: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Minimum value
    pub min: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Minimum length of value
    pub minlength: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to allow multiple values
    pub multiple: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Name of the element to use for form submission and in the form.elements API
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Pattern to be matched by the form control's value
    pub pattern: std::option::Option<std::borrow::Cow<'static, str>>,
    /// User-visible label to be placed within the form control
    pub placeholder: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Targets a popover element to toggle, show, or hide
    pub popovertarget: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden
    pub popovertargetaction: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to allow the value to be edited by the user
    pub readonly: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the control is required for form submission
    pub required: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Size of the control
    pub size: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Granularity to be matched by the form control's value
    pub step: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Type of form control
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Value of the form control
    pub value: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Horizontal dimension
    pub width: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Input {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<input")?;
        if let Some(field) = self.accept.as_ref() {
            write!(writer, r#" accept="{field}""#)?;
        }
        if let Some(field) = self.alt.as_ref() {
            write!(writer, r#" alt="{field}""#)?;
        }
        if let Some(field) = self.autocomplete.as_ref() {
            write!(writer, r#" autocomplete="{field}""#)?;
        }
        if let Some(field) = self.checked.as_ref() {
            write!(writer, r#" checked="{field}""#)?;
        }
        if let Some(field) = self.dirname.as_ref() {
            write!(writer, r#" dirname="{field}""#)?;
        }
        if let Some(field) = self.disabled.as_ref() {
            write!(writer, r#" disabled="{field}""#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#" form="{field}""#)?;
        }
        if let Some(field) = self.formaction.as_ref() {
            write!(writer, r#" formaction="{field}""#)?;
        }
        if let Some(field) = self.formenctype.as_ref() {
            write!(writer, r#" formenctype="{field}""#)?;
        }
        if let Some(field) = self.formmethod.as_ref() {
            write!(writer, r#" formmethod="{field}""#)?;
        }
        if let Some(field) = self.formnovalidate.as_ref() {
            write!(writer, r#" formnovalidate="{field}""#)?;
        }
        if let Some(field) = self.formtarget.as_ref() {
            write!(writer, r#" formtarget="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#" height="{field}""#)?;
        }
        if let Some(field) = self.list.as_ref() {
            write!(writer, r#" list="{field}""#)?;
        }
        if let Some(field) = self.max.as_ref() {
            write!(writer, r#" max="{field}""#)?;
        }
        if let Some(field) = self.maxlength.as_ref() {
            write!(writer, r#" maxlength="{field}""#)?;
        }
        if let Some(field) = self.min.as_ref() {
            write!(writer, r#" min="{field}""#)?;
        }
        if let Some(field) = self.minlength.as_ref() {
            write!(writer, r#" minlength="{field}""#)?;
        }
        if let Some(field) = self.multiple.as_ref() {
            write!(writer, r#" multiple="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        if let Some(field) = self.pattern.as_ref() {
            write!(writer, r#" pattern="{field}""#)?;
        }
        if let Some(field) = self.placeholder.as_ref() {
            write!(writer, r#" placeholder="{field}""#)?;
        }
        if let Some(field) = self.popovertarget.as_ref() {
            write!(writer, r#" popovertarget="{field}""#)?;
        }
        if let Some(field) = self.popovertargetaction.as_ref() {
            write!(writer, r#" popovertargetaction="{field}""#)?;
        }
        if let Some(field) = self.readonly.as_ref() {
            write!(writer, r#" readonly="{field}""#)?;
        }
        if let Some(field) = self.required.as_ref() {
            write!(writer, r#" required="{field}""#)?;
        }
        if let Some(field) = self.size.as_ref() {
            write!(writer, r#" size="{field}""#)?;
        }
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.step.as_ref() {
            write!(writer, r#" step="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#" type="{field}""#)?;
        }
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#" value="{field}""#)?;
        }
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#" width="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::fmt::Display for Input {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Input {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
