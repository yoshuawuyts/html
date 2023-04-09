/// The HTML `<data>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
#[doc(alias = "data")]
#[non_exhaustive]
pub struct Data<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Data,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Data<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Data<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Data<T> {}
