/// The HTML `<label>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
#[doc(alias = "label")]
#[non_exhaustive]
pub struct Label {
    _sys: html_sys::forms::Label,
}
impl crate::categories::FlowContent for Label {}
impl crate::categories::PhrasingContent for Label {}
impl crate::categories::InteractiveContent for Label {}
impl crate::categories::PalpableContent for Label {}
