/// The HTML `<textarea>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[non_exhaustive]
pub struct TextArea {
    sys: html_sys::forms::TextArea,
}
impl TextArea {
    /// Get the value of the `autocomplete` attribute
    pub fn autocomplete(&self) -> std::option::Option<&str> {
        self.sys.autocomplete.as_deref()
    }
    /// Set the value of the `autocomplete` attribute
    pub fn set_autocomplete(&mut self, value: std::option::Option<String>) {
        self.sys.autocomplete = value;
    }
    /// Get the value of the `cols` attribute
    pub fn cols(&self) -> std::option::Option<&str> {
        self.sys.cols.as_deref()
    }
    /// Set the value of the `cols` attribute
    pub fn set_cols(&mut self, value: std::option::Option<String>) {
        self.sys.cols = value;
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
    /// Get the value of the `maxlength` attribute
    pub fn maxlength(&self) -> std::option::Option<&str> {
        self.sys.maxlength.as_deref()
    }
    /// Set the value of the `maxlength` attribute
    pub fn set_maxlength(&mut self, value: std::option::Option<String>) {
        self.sys.maxlength = value;
    }
    /// Get the value of the `minlength` attribute
    pub fn minlength(&self) -> std::option::Option<&str> {
        self.sys.minlength.as_deref()
    }
    /// Set the value of the `minlength` attribute
    pub fn set_minlength(&mut self, value: std::option::Option<String>) {
        self.sys.minlength = value;
    }
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
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
    /// Get the value of the `rows` attribute
    pub fn rows(&self) -> std::option::Option<&str> {
        self.sys.rows.as_deref()
    }
    /// Set the value of the `rows` attribute
    pub fn set_rows(&mut self, value: std::option::Option<String>) {
        self.sys.rows = value;
    }
    /// Get the value of the `wrap` attribute
    pub fn wrap(&self) -> std::option::Option<&str> {
        self.sys.wrap.as_deref()
    }
    /// Set the value of the `wrap` attribute
    pub fn set_wrap(&mut self, value: std::option::Option<String>) {
        self.sys.wrap = value;
    }
}
impl crate::categories::FlowContent for TextArea {}
impl crate::categories::PhrasingContent for TextArea {}
impl crate::categories::InteractiveContent for TextArea {}
impl crate::categories::PalpableContent for TextArea {}
impl std::convert::Into<html_sys::forms::TextArea> for TextArea {
    fn into(self) -> html_sys::forms::TextArea {
        self.sys
    }
}
impl From<html_sys::forms::TextArea> for TextArea {
    fn from(sys: html_sys::forms::TextArea) -> Self {
        Self { sys }
    }
}
