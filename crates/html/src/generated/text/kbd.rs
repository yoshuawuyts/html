/// The HTML `<kbd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
#[doc(alias = "kbd")]
#[non_exhaustive]
pub struct KeyboardInput<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::KeyboardInput,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for KeyboardInput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for KeyboardInput<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for KeyboardInput<T> {}
