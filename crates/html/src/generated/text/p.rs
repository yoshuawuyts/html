/// The HTML `<p>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
#[doc(alias = "p")]
#[non_exhaustive]
pub struct Paragraph<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Paragraph,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Paragraph<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Paragraph<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Paragraph>
for Paragraph<T> {
    fn into(self) -> html_sys::text::Paragraph {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Paragraph>
for Paragraph<T> {
    fn from(sys: html_sys::text::Paragraph) -> Self {
        Self { sys, _children: vec![] }
    }
}
