/// The HTML `<output>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
#[doc(alias = "output")]
#[non_exhaustive]
pub struct Output<T: crate::categories::PhrasingContent> {
    sys: html_sys::forms::Output,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Output<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Output<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Output<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::forms::Output>
for Output<T> {
    fn into(self) -> html_sys::forms::Output {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::forms::Output> for Output<T> {
    fn from(sys: html_sys::forms::Output) -> Self {
        Self { sys, _children: vec![] }
    }
}
