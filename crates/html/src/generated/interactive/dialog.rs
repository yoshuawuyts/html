/// The HTML `<dialog>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
#[doc(alias = "dialog")]
#[non_exhaustive]
pub struct Dialog<T: crate::categories::FlowContent> {
    _sys: html_sys::interactive::Dialog,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Dialog<T> {}
