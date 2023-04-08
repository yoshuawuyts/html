/// The HTML `<aside>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
#[doc(alias = "aside")]
#[non_exhaustive]
pub struct Aside {
    _sys: html_sys::sections::Aside,
}
impl crate::categories::FlowContent for Aside {}
impl crate::categories::SectioningContent for Aside {}
impl crate::categories::PalpableContent for Aside {}
