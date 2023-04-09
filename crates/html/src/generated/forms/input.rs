/// The HTML `<input>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[doc(alias = "input")]
#[non_exhaustive]
pub struct Input {
    sys: html_sys::forms::Input,
    _children: Vec<()>,
}
impl Input {
    /// Get the value of the `accept` attribute
    pub fn accept(&self) -> std::option::Option<&str> {
        self.sys.accept.as_deref()
    }
    /// Set the value of the `accept` attribute
    pub fn set_accept(&mut self, value: std::option::Option<String>) {
        self.sys.accept = value;
    }
    /// Get the value of the `alt` attribute
    pub fn alt(&self) -> std::option::Option<&str> {
        self.sys.alt.as_deref()
    }
    /// Set the value of the `alt` attribute
    pub fn set_alt(&mut self, value: std::option::Option<String>) {
        self.sys.alt = value;
    }
    /// Get the value of the `autocomplete` attribute
    pub fn autocomplete(&self) -> std::option::Option<&str> {
        self.sys.autocomplete.as_deref()
    }
    /// Set the value of the `autocomplete` attribute
    pub fn set_autocomplete(&mut self, value: std::option::Option<String>) {
        self.sys.autocomplete = value;
    }
    /// Get the value of the `checked` attribute
    pub fn checked(&self) -> std::option::Option<&str> {
        self.sys.checked.as_deref()
    }
    /// Set the value of the `checked` attribute
    pub fn set_checked(&mut self, value: std::option::Option<String>) {
        self.sys.checked = value;
    }
    /// Get the value of the `dirname` attribute
    pub fn dirname(&self) -> std::option::Option<&str> {
        self.sys.dirname.as_deref()
    }
    /// Set the value of the `dirname` attribute
    pub fn set_dirname(&mut self, value: std::option::Option<String>) {
        self.sys.dirname = value;
    }
    /// Get the value of the `disabled` attribute
    pub fn disabled(&self) -> std::option::Option<&str> {
        self.sys.disabled.as_deref()
    }
    /// Set the value of the `disabled` attribute
    pub fn set_disabled(&mut self, value: std::option::Option<String>) {
        self.sys.disabled = value;
    }
    /// Get the value of the `form` attribute
    pub fn form(&self) -> std::option::Option<&str> {
        self.sys.form.as_deref()
    }
    /// Set the value of the `form` attribute
    pub fn set_form(&mut self, value: std::option::Option<String>) {
        self.sys.form = value;
    }
    /// Get the value of the `formaction` attribute
    pub fn formaction(&self) -> std::option::Option<&str> {
        self.sys.formaction.as_deref()
    }
    /// Set the value of the `formaction` attribute
    pub fn set_formaction(&mut self, value: std::option::Option<String>) {
        self.sys.formaction = value;
    }
    /// Get the value of the `formenctype` attribute
    pub fn formenctype(&self) -> std::option::Option<&str> {
        self.sys.formenctype.as_deref()
    }
    /// Set the value of the `formenctype` attribute
    pub fn set_formenctype(&mut self, value: std::option::Option<String>) {
        self.sys.formenctype = value;
    }
    /// Get the value of the `formmethod` attribute
    pub fn formmethod(&self) -> std::option::Option<&str> {
        self.sys.formmethod.as_deref()
    }
    /// Set the value of the `formmethod` attribute
    pub fn set_formmethod(&mut self, value: std::option::Option<String>) {
        self.sys.formmethod = value;
    }
    /// Get the value of the `formnovalidate` attribute
    pub fn formnovalidate(&self) -> std::option::Option<&str> {
        self.sys.formnovalidate.as_deref()
    }
    /// Set the value of the `formnovalidate` attribute
    pub fn set_formnovalidate(&mut self, value: std::option::Option<String>) {
        self.sys.formnovalidate = value;
    }
    /// Get the value of the `formtarget` attribute
    pub fn formtarget(&self) -> std::option::Option<&str> {
        self.sys.formtarget.as_deref()
    }
    /// Set the value of the `formtarget` attribute
    pub fn set_formtarget(&mut self, value: std::option::Option<String>) {
        self.sys.formtarget = value;
    }
    /// Get the value of the `height` attribute
    pub fn height(&self) -> std::option::Option<&str> {
        self.sys.height.as_deref()
    }
    /// Set the value of the `height` attribute
    pub fn set_height(&mut self, value: std::option::Option<String>) {
        self.sys.height = value;
    }
    /// Get the value of the `list` attribute
    pub fn list(&self) -> std::option::Option<&str> {
        self.sys.list.as_deref()
    }
    /// Set the value of the `list` attribute
    pub fn set_list(&mut self, value: std::option::Option<String>) {
        self.sys.list = value;
    }
    /// Get the value of the `max` attribute
    pub fn max(&self) -> std::option::Option<&str> {
        self.sys.max.as_deref()
    }
    /// Set the value of the `max` attribute
    pub fn set_max(&mut self, value: std::option::Option<String>) {
        self.sys.max = value;
    }
    /// Get the value of the `maxlength` attribute
    pub fn maxlength(&self) -> std::option::Option<&str> {
        self.sys.maxlength.as_deref()
    }
    /// Set the value of the `maxlength` attribute
    pub fn set_maxlength(&mut self, value: std::option::Option<String>) {
        self.sys.maxlength = value;
    }
    /// Get the value of the `min` attribute
    pub fn min(&self) -> std::option::Option<&str> {
        self.sys.min.as_deref()
    }
    /// Set the value of the `min` attribute
    pub fn set_min(&mut self, value: std::option::Option<String>) {
        self.sys.min = value;
    }
    /// Get the value of the `minlength` attribute
    pub fn minlength(&self) -> std::option::Option<&str> {
        self.sys.minlength.as_deref()
    }
    /// Set the value of the `minlength` attribute
    pub fn set_minlength(&mut self, value: std::option::Option<String>) {
        self.sys.minlength = value;
    }
    /// Get the value of the `multiple` attribute
    pub fn multiple(&self) -> std::option::Option<&str> {
        self.sys.multiple.as_deref()
    }
    /// Set the value of the `multiple` attribute
    pub fn set_multiple(&mut self, value: std::option::Option<String>) {
        self.sys.multiple = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
    /// Get the value of the `pattern` attribute
    pub fn pattern(&self) -> std::option::Option<&str> {
        self.sys.pattern.as_deref()
    }
    /// Set the value of the `pattern` attribute
    pub fn set_pattern(&mut self, value: std::option::Option<String>) {
        self.sys.pattern = value;
    }
    /// Get the value of the `placeholder` attribute
    pub fn placeholder(&self) -> std::option::Option<&str> {
        self.sys.placeholder.as_deref()
    }
    /// Set the value of the `placeholder` attribute
    pub fn set_placeholder(&mut self, value: std::option::Option<String>) {
        self.sys.placeholder = value;
    }
    /// Get the value of the `readonly` attribute
    pub fn readonly(&self) -> std::option::Option<&str> {
        self.sys.readonly.as_deref()
    }
    /// Set the value of the `readonly` attribute
    pub fn set_readonly(&mut self, value: std::option::Option<String>) {
        self.sys.readonly = value;
    }
    /// Get the value of the `required` attribute
    pub fn required(&self) -> std::option::Option<&str> {
        self.sys.required.as_deref()
    }
    /// Set the value of the `required` attribute
    pub fn set_required(&mut self, value: std::option::Option<String>) {
        self.sys.required = value;
    }
    /// Get the value of the `size` attribute
    pub fn size(&self) -> std::option::Option<&str> {
        self.sys.size.as_deref()
    }
    /// Set the value of the `size` attribute
    pub fn set_size(&mut self, value: std::option::Option<String>) {
        self.sys.size = value;
    }
    /// Get the value of the `src` attribute
    pub fn src(&self) -> std::option::Option<&str> {
        self.sys.src.as_deref()
    }
    /// Set the value of the `src` attribute
    pub fn set_src(&mut self, value: std::option::Option<String>) {
        self.sys.src = value;
    }
    /// Get the value of the `step` attribute
    pub fn step(&self) -> std::option::Option<&str> {
        self.sys.step.as_deref()
    }
    /// Set the value of the `step` attribute
    pub fn set_step(&mut self, value: std::option::Option<String>) {
        self.sys.step = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
    /// Get the value of the `value` attribute
    pub fn value(&self) -> std::option::Option<&str> {
        self.sys.value.as_deref()
    }
    /// Set the value of the `value` attribute
    pub fn set_value(&mut self, value: std::option::Option<String>) {
        self.sys.value = value;
    }
    /// Get the value of the `width` attribute
    pub fn width(&self) -> std::option::Option<&str> {
        self.sys.width.as_deref()
    }
    /// Set the value of the `width` attribute
    pub fn set_width(&mut self, value: std::option::Option<String>) {
        self.sys.width = value;
    }
}
impl crate::categories::FlowContent for Input {}
impl crate::categories::PhrasingContent for Input {}
impl std::convert::Into<html_sys::forms::Input> for Input {
    fn into(self) -> html_sys::forms::Input {
        self.sys
    }
}
impl From<html_sys::forms::Input> for Input {
    fn from(sys: html_sys::forms::Input) -> Self {
        Self { sys, _children: vec![] }
    }
}
