/// The HTML `<strong>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
#[doc(alias = "strong")]
#[non_exhaustive]
pub struct Strong {
    _sys: html_sys::text::Strong,
}
impl crate::categories::FlowContent for Strong {}
impl crate::categories::PhrasingContent for Strong {}
impl crate::categories::PalpableContent for Strong {}
