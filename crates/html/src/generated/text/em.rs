/// The HTML `<em>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
#[doc(alias = "em")]
#[non_exhaustive]
pub struct Emphasis<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Emphasis,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Emphasis<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Emphasis<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Emphasis<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Emphasis>
for Emphasis<T> {
    fn into(self) -> html_sys::text::Emphasis {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Emphasis>
for Emphasis<T> {
    fn from(sys: html_sys::text::Emphasis) -> Self {
        Self { sys, _children: vec![] }
    }
}
