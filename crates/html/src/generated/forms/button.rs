/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
pub struct Button {
    sys: html_sys::forms::Button,
}
impl crate::categories::FlowContent for Button {}
impl crate::categories::PhrasingContent for Button {}
impl crate::categories::InteractiveContent for Button {}
impl crate::categories::PalpableContent for Button {}
impl std::convert::Into<html_sys::forms::Button> for Button {
    fn into(self) -> html_sys::forms::Button {
        self.sys
    }
}
impl From<html_sys::forms::Button> for Button {
    fn from(sys: html_sys::forms::Button) -> Self {
        Self { sys }
    }
}
