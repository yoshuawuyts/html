/// The HTML `<del>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
#[doc(alias = "del")]
#[non_exhaustive]
pub struct DeletedText {
    _sys: html_sys::edits::DeletedText,
}
impl crate::categories::FlowContent for DeletedText {}
impl crate::categories::PhrasingContent for DeletedText {}
impl crate::categories::PalpableContent for DeletedText {}
