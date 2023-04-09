/// The HTML `<kbd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
#[doc(alias = "kbd")]
#[non_exhaustive]
pub struct KeyboardInput {
    sys: html_sys::text::KeyboardInput,
    _children: Vec<()>,
}
impl crate::HtmlElement for KeyboardInput {}
impl crate::categories::FlowContent for KeyboardInput {}
impl crate::categories::PhrasingContent for KeyboardInput {}
impl crate::categories::PalpableContent for KeyboardInput {}
impl std::convert::Into<html_sys::text::KeyboardInput> for KeyboardInput {
    fn into(self) -> html_sys::text::KeyboardInput {
        self.sys
    }
}
impl From<html_sys::text::KeyboardInput> for KeyboardInput {
    fn from(sys: html_sys::text::KeyboardInput) -> Self {
        Self { sys, _children: vec![] }
    }
}
