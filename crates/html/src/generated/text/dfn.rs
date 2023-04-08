/// The HTML `<dfn>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn)
#[doc(alias = "dfn")]
#[non_exhaustive]
pub struct Definition {
    _sys: html_sys::text::Definition,
}
impl crate::categories::FlowContent for Definition {}
impl crate::categories::PhrasingContent for Definition {}
impl crate::categories::PalpableContent for Definition {}
