/// The HTML `<input>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[doc(alias = "input")]
#[non_exhaustive]
pub struct Input {
    sys: html_sys::forms::Input,
}
impl crate::categories::FlowContent for Input {}
impl crate::categories::PhrasingContent for Input {}
impl std::convert::Into<html_sys::forms::Input> for Input {
    fn into(self) -> html_sys::forms::Input {
        self.sys
    }
}
impl From<html_sys::forms::Input> for Input {
    fn from(sys: html_sys::forms::Input) -> Self {
        Self { sys }
    }
}
