/// The HTML `<slot>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
#[doc(alias = "slot")]
#[non_exhaustive]
pub struct Slot {
    _sys: html_sys::scripting::Slot,
}
impl crate::categories::FlowContent for Slot {}
impl crate::categories::PhrasingContent for Slot {}
