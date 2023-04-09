/// The HTML `<span>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
#[doc(alias = "span")]
#[non_exhaustive]
pub struct Span<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Span,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Span<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Span<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Span<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Span>
for Span<T> {
    fn into(self) -> html_sys::text::Span {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Span> for Span<T> {
    fn from(sys: html_sys::text::Span) -> Self {
        Self { sys, _children: vec![] }
    }
}
