/// The HTML `<textarea>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[non_exhaustive]
pub struct TextArea {
    sys: html_sys::forms::TextArea,
}
impl crate::categories::FlowContent for TextArea {}
impl crate::categories::PhrasingContent for TextArea {}
impl crate::categories::InteractiveContent for TextArea {}
impl crate::categories::PalpableContent for TextArea {}
impl std::convert::Into<html_sys::forms::TextArea> for TextArea {
    fn into(self) -> html_sys::forms::TextArea {
        self.sys
    }
}
impl From<html_sys::forms::TextArea> for TextArea {
    fn from(sys: html_sys::forms::TextArea) -> Self {
        Self { sys }
    }
}
