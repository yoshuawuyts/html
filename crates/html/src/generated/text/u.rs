/// The HTML `<u>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
#[doc(alias = "u")]
#[non_exhaustive]
pub struct Underline<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Underline,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Underline<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Underline<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Underline<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Underline>
for Underline<T> {
    fn into(self) -> html_sys::text::Underline {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Underline>
for Underline<T> {
    fn from(sys: html_sys::text::Underline) -> Self {
        Self { sys, _children: vec![] }
    }
}
