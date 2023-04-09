/// The HTML `<section>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
#[doc(alias = "section")]
#[non_exhaustive]
pub struct Section {
    sys: html_sys::sections::Section,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Section {}
impl crate::categories::SectioningContent for Section {}
impl crate::categories::PalpableContent for Section {}
impl std::convert::Into<html_sys::sections::Section> for Section {
    fn into(self) -> html_sys::sections::Section {
        self.sys
    }
}
impl From<html_sys::sections::Section> for Section {
    fn from(sys: html_sys::sections::Section) -> Self {
        Self { sys, _children: vec![] }
    }
}
