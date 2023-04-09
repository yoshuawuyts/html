/// The HTML `<cite>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
#[doc(alias = "cite")]
#[non_exhaustive]
pub struct Cite<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Cite,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Cite<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Cite<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Cite<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Cite>
for Cite<T> {
    fn into(self) -> html_sys::text::Cite {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Cite> for Cite<T> {
    fn from(sys: html_sys::text::Cite) -> Self {
        Self { sys, _children: vec![] }
    }
}
