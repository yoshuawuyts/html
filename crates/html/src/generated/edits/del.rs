/// The HTML `<del>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
#[doc(alias = "del")]
#[non_exhaustive]
pub struct DeletedText {
    sys: html_sys::edits::DeletedText,
}
impl DeletedText {
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
impl crate::categories::FlowContent for DeletedText {}
impl crate::categories::PhrasingContent for DeletedText {}
impl crate::categories::PalpableContent for DeletedText {}
impl std::convert::Into<html_sys::edits::DeletedText> for DeletedText {
    fn into(self) -> html_sys::edits::DeletedText {
        self.sys
    }
}
impl From<html_sys::edits::DeletedText> for DeletedText {
    fn from(sys: html_sys::edits::DeletedText) -> Self {
        Self { sys }
    }
}
