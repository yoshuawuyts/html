/// The HTML `<ins>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins)
#[doc(alias = "ins")]
#[non_exhaustive]
pub struct InsertedText {
    sys: html_sys::edits::InsertedText,
}
impl InsertedText {
    /// Get the value of the `cite` attribute
    pub fn cite(&self) -> std::option::Option<&str> {
        self.sys.cite.as_deref()
    }
    /// Set the value of the `cite` attribute
    pub fn set_cite(&mut self, value: std::option::Option<String>) {
        self.sys.cite = value;
    }
    /// Get the value of the `datetime` attribute
    pub fn datetime(&self) -> std::option::Option<&str> {
        self.sys.datetime.as_deref()
    }
    /// Set the value of the `datetime` attribute
    pub fn set_datetime(&mut self, value: std::option::Option<String>) {
        self.sys.datetime = value;
    }
}
impl crate::categories::FlowContent for InsertedText {}
impl crate::categories::PhrasingContent for InsertedText {}
impl crate::categories::PalpableContent for InsertedText {}
impl std::convert::Into<html_sys::edits::InsertedText> for InsertedText {
    fn into(self) -> html_sys::edits::InsertedText {
        self.sys
    }
}
impl From<html_sys::edits::InsertedText> for InsertedText {
    fn from(sys: html_sys::edits::InsertedText) -> Self {
        Self { sys }
    }
}
