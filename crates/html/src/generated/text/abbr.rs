/// The HTML `<abbr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
#[doc(alias = "abbr")]
#[non_exhaustive]
pub struct Abbreviation {
    _sys: html_sys::text::Abbreviation,
}
impl crate::categories::FlowContent for Abbreviation {}
impl crate::categories::PhrasingContent for Abbreviation {}
impl crate::categories::PalpableContent for Abbreviation {}
