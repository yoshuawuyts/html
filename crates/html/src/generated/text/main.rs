/// The HTML `<main>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main)
#[doc(alias = "main")]
#[non_exhaustive]
pub struct Main<T: crate::categories::FlowContent> {
    _sys: html_sys::text::Main,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Main<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent for Main<T> {}
