/// The HTML `<fieldset>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
#[doc(alias = "fieldset")]
#[non_exhaustive]
pub struct Fieldset {
    _sys: html_sys::forms::Fieldset,
}
impl crate::categories::FlowContent for Fieldset {}
impl crate::categories::PalpableContent for Fieldset {}
