/// The HTML `<textarea>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[non_exhaustive]
pub struct TextArea {
    sys: html_sys::forms::TextArea,
    _children: Vec<()>,
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
    pub fn cols(&self) -> std::option::Option<i64> {
        self.sys.cols
    }
    /// Set the value of the `cols` attribute
    pub fn set_cols(&mut self, value: std::option::Option<i64>) {
        self.sys.cols = value;
    }
    /// Get the value of the `dirname` attribute
    pub fn dir_name(&self) -> std::option::Option<&str> {
        self.sys.dir_name.as_deref()
    }
    /// Set the value of the `dirname` attribute
    pub fn set_dir_name(&mut self, value: std::option::Option<String>) {
        self.sys.dir_name = value;
    }
    /// Get the value of the `disabled` attribute
    pub fn disabled(&self) -> bool {
        self.sys.disabled
    }
    /// Set the value of the `disabled` attribute
    pub fn set_disabled(&mut self, value: bool) {
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
    pub fn max_length(&self) -> std::option::Option<i64> {
        self.sys.max_length
    }
    /// Set the value of the `maxlength` attribute
    pub fn set_max_length(&mut self, value: std::option::Option<i64>) {
        self.sys.max_length = value;
    }
    /// Get the value of the `minlength` attribute
    pub fn min_length(&self) -> std::option::Option<i64> {
        self.sys.min_length
    }
    /// Set the value of the `minlength` attribute
    pub fn set_min_length(&mut self, value: std::option::Option<i64>) {
        self.sys.min_length = value;
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
    pub fn read_only(&self) -> bool {
        self.sys.read_only
    }
    /// Set the value of the `readonly` attribute
    pub fn set_read_only(&mut self, value: bool) {
        self.sys.read_only = value;
    }
    /// Get the value of the `required` attribute
    pub fn required(&self) -> bool {
        self.sys.required
    }
    /// Set the value of the `required` attribute
    pub fn set_required(&mut self, value: bool) {
        self.sys.required = value;
    }
    /// Get the value of the `rows` attribute
    pub fn rows(&self) -> std::option::Option<i64> {
        self.sys.rows
    }
    /// Set the value of the `rows` attribute
    pub fn set_rows(&mut self, value: std::option::Option<i64>) {
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
        Self { sys, _children: vec![] }
    }
}
