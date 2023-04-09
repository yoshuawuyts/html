/// The HTML `<kbd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
#[doc(alias = "kbd")]
#[non_exhaustive]
pub struct KeyboardInput<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::KeyboardInput,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for KeyboardInput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for KeyboardInput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for KeyboardInput<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::KeyboardInput> for KeyboardInput<T> {
    fn into(self) -> html_sys::text::KeyboardInput {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::KeyboardInput>
for KeyboardInput<T> {
    fn from(sys: html_sys::text::KeyboardInput) -> Self {
        Self { sys, _children: vec![] }
    }
}
