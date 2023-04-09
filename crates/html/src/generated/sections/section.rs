/// The HTML `<section>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
#[doc(alias = "section")]
#[non_exhaustive]
pub struct Section<T: crate::categories::FlowContent> {
    sys: html_sys::sections::Section,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Section<T> {}
impl<T: crate::categories::FlowContent> crate::categories::SectioningContent
for Section<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for Section<T> {}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::sections::Section>
for Section<T> {
    fn into(self) -> html_sys::sections::Section {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::sections::Section>
for Section<T> {
    fn from(sys: html_sys::sections::Section) -> Self {
        Self { sys, _children: vec![] }
    }
}
