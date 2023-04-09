/// The HTML `<var>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
#[doc(alias = "var")]
#[non_exhaustive]
pub struct Variable<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Variable,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Variable<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Variable<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Variable<T> {}
