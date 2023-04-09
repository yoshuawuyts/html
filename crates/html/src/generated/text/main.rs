/// The HTML `<main>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main)
#[doc(alias = "main")]
#[non_exhaustive]
pub struct Main<T: crate::categories::FlowContent> {
    sys: html_sys::text::Main,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent for Main<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent for Main<T> {}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::text::Main>
for Main<T> {
    fn into(self) -> html_sys::text::Main {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::text::Main> for Main<T> {
    fn from(sys: html_sys::text::Main) -> Self {
        Self { sys, _children: vec![] }
    }
}
