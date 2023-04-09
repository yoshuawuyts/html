/// The HTML `<strong>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
#[doc(alias = "strong")]
#[non_exhaustive]
pub struct Strong<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Strong,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Strong<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Strong<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Strong<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Strong>
for Strong<T> {
    fn into(self) -> html_sys::text::Strong {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Strong> for Strong<T> {
    fn from(sys: html_sys::text::Strong) -> Self {
        Self { sys, _children: vec![] }
    }
}
