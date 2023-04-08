/// The HTML `<em>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
#[doc(alias = "em")]
#[non_exhaustive]
pub struct Emphasis {
    _sys: html_sys::text::Emphasis,
}
impl crate::categories::FlowContent for Emphasis {}
impl crate::categories::PhrasingContent for Emphasis {}
impl crate::categories::PalpableContent for Emphasis {}
