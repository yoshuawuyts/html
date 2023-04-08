/// The HTML `<ins>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins)
#[doc(alias = "ins")]
#[non_exhaustive]
pub struct InsertedText {
    _sys: html_sys::edits::InsertedText,
}
impl crate::categories::FlowContent for InsertedText {}
impl crate::categories::PhrasingContent for InsertedText {}
impl crate::categories::PalpableContent for InsertedText {}
