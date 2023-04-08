/// The HTML `<section>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
#[doc(alias = "section")]
#[non_exhaustive]
pub struct Section {
    _sys: html_sys::sections::Section,
}
impl crate::categories::FlowContent for Section {}
impl crate::categories::SectioningContent for Section {}
impl crate::categories::PalpableContent for Section {}
