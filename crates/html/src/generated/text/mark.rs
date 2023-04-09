/// The HTML `<mark>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
#[doc(alias = "mark")]
#[non_exhaustive]
pub struct MarkText {
    sys: html_sys::text::MarkText,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for MarkText {}
impl crate::categories::PhrasingContent for MarkText {}
impl crate::categories::PalpableContent for MarkText {}
impl std::convert::Into<html_sys::text::MarkText> for MarkText {
    fn into(self) -> html_sys::text::MarkText {
        self.sys
    }
}
impl From<html_sys::text::MarkText> for MarkText {
    fn from(sys: html_sys::text::MarkText) -> Self {
        Self { sys, _children: vec![] }
    }
}
