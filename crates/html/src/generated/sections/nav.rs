/// The HTML `<nav>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
#[doc(alias = "nav")]
#[non_exhaustive]
pub struct Navigation {
    _sys: html_sys::sections::Navigation,
}
impl crate::categories::FlowContent for Navigation {}
impl crate::categories::SectioningContent for Navigation {}
impl crate::categories::PalpableContent for Navigation {}
