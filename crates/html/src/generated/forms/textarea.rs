/// The HTML `<textarea>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[non_exhaustive]
pub struct TextArea {
    _sys: html_sys::forms::TextArea,
}
impl crate::categories::FlowContent for TextArea {}
impl crate::categories::PhrasingContent for TextArea {}
impl crate::categories::InteractiveContent for TextArea {}
impl crate::categories::PalpableContent for TextArea {}
