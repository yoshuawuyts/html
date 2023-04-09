/// The HTML `<dialog>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
#[doc(alias = "dialog")]
#[non_exhaustive]
pub struct Dialog {
    sys: html_sys::interactive::Dialog,
    _children: Vec<()>,
}
impl Dialog {
    /// Get the value of the `open` attribute
    pub fn open(&self) -> bool {
        self.sys.open
    }
    /// Set the value of the `open` attribute
    pub fn set_open(&mut self, value: bool) {
        self.sys.open = value;
    }
}
impl crate::HtmlElement for Dialog {}
impl crate::categories::FlowContent for Dialog {}
impl std::convert::Into<html_sys::interactive::Dialog> for Dialog {
    fn into(self) -> html_sys::interactive::Dialog {
        self.sys
    }
}
impl From<html_sys::interactive::Dialog> for Dialog {
    fn from(sys: html_sys::interactive::Dialog) -> Self {
        Self { sys, _children: vec![] }
    }
}
