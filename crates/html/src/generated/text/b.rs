/// The HTML `<b>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
#[doc(alias = "b")]
#[non_exhaustive]
pub struct Bold<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Bold,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Bold<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Bold<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Bold<T> {}
