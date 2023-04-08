/// The HTML `<var>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
#[doc(alias = "var")]
#[non_exhaustive]
pub struct Variable {
    _sys: html_sys::text::Variable,
}
impl crate::categories::FlowContent for Variable {}
impl crate::categories::PhrasingContent for Variable {}
impl crate::categories::PalpableContent for Variable {}
