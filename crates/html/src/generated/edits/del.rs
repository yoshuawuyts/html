/// The HTML `<del>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
#[doc(alias = "del")]
#[non_exhaustive]
pub struct DeletedText {
    sys: html_sys::edits::DeletedText,
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
