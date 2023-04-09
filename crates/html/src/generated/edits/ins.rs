/// The HTML `<ins>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins)
#[doc(alias = "ins")]
#[non_exhaustive]
pub struct InsertedText {
    sys: html_sys::edits::InsertedText,
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
