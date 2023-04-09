/// The HTML `<h3>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3)
#[doc(alias = "h3")]
#[non_exhaustive]
pub struct Heading3<T: crate::categories::PhrasingContent> {
    sys: html_sys::sections::Heading3,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading3<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading3<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading3<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::sections::Heading3> for Heading3<T> {
    fn into(self) -> html_sys::sections::Heading3 {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::sections::Heading3>
for Heading3<T> {
    fn from(sys: html_sys::sections::Heading3) -> Self {
        Self { sys, _children: vec![] }
    }
}
