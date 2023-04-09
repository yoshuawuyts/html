/// The HTML `<sub>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
#[doc(alias = "sub")]
#[non_exhaustive]
pub struct SubScript<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::SubScript,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SubScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SubScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SubScript<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::SubScript>
for SubScript<T> {
    fn into(self) -> html_sys::text::SubScript {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::SubScript>
for SubScript<T> {
    fn from(sys: html_sys::text::SubScript) -> Self {
        Self { sys, _children: vec![] }
    }
}
