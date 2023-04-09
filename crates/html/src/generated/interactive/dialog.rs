/// The HTML `<dialog>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
#[doc(alias = "dialog")]
#[non_exhaustive]
pub struct Dialog<T: crate::categories::FlowContent> {
    sys: html_sys::interactive::Dialog,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Dialog<T> {}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::interactive::Dialog>
for Dialog<T> {
    fn into(self) -> html_sys::interactive::Dialog {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::interactive::Dialog>
for Dialog<T> {
    fn from(sys: html_sys::interactive::Dialog) -> Self {
        Self { sys, _children: vec![] }
    }
}
