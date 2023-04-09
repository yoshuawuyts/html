/// The HTML `<code>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
#[doc(alias = "code")]
#[non_exhaustive]
pub struct Code<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Code,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Code<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Code<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Code<T> {}
