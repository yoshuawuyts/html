/// The HTML `<mark>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
#[doc(alias = "mark")]
#[non_exhaustive]
pub struct MarkText<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::MarkText,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for MarkText<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for MarkText<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for MarkText<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::MarkText>
for MarkText<T> {
    fn into(self) -> html_sys::text::MarkText {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::MarkText>
for MarkText<T> {
    fn from(sys: html_sys::text::MarkText) -> Self {
        Self { sys, _children: vec![] }
    }
}
