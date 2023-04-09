/// The HTML `<h1>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1)
#[doc(alias = "h1")]
#[non_exhaustive]
pub struct Heading1<T: crate::categories::PhrasingContent> {
    sys: html_sys::sections::Heading1,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading1<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading1<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading1<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::sections::Heading1> for Heading1<T> {
    fn into(self) -> html_sys::sections::Heading1 {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::sections::Heading1>
for Heading1<T> {
    fn from(sys: html_sys::sections::Heading1) -> Self {
        Self { sys, _children: vec![] }
    }
}
