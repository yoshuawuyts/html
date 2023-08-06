/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Button {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Whether the form control is disabled
    pub disabled: bool,
    /// Associates the element with a form element
    pub form: std::option::Option<std::borrow::Cow<'static, str>>,
    /// URL to use for form submission
    pub form_action: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Entry list encoding type to use for form submission
    pub form_enctype: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Variant to use for form submission
    pub form_method: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Bypass form control validation for form submission
    pub form_no_validate: bool,
    /// Navigable for form submission
    pub form_target: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Name of the element to use for form submission and in the form.elements API
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Targets a popover element to toggle, show, or hide
    pub popovertarget: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden
    pub popovertargetaction: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Type of button
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Value to be used for form submission
    pub value: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Button {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<button")?;
        if self.disabled {
            write!(writer, r#" disabled"#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#" form="{field}""#)?;
        }
        if let Some(field) = self.form_action.as_ref() {
            write!(writer, r#" formaction="{field}""#)?;
        }
        if let Some(field) = self.form_enctype.as_ref() {
            write!(writer, r#" formenctype="{field}""#)?;
        }
        if let Some(field) = self.form_method.as_ref() {
            write!(writer, r#" formmethod="{field}""#)?;
        }
        if self.form_no_validate {
            write!(writer, r#" formnovalidate"#)?;
        }
        if let Some(field) = self.form_target.as_ref() {
            write!(writer, r#" formtarget="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        if let Some(field) = self.popovertarget.as_ref() {
            write!(writer, r#" popovertarget="{field}""#)?;
        }
        if let Some(field) = self.popovertargetaction.as_ref() {
            write!(writer, r#" popovertargetaction="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#" type="{field}""#)?;
        }
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#" value="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</button>")?;
        Ok(())
    }
}
impl std::fmt::Display for Button {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Button {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
