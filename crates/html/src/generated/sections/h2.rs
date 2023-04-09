/// The HTML `<h2>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
#[doc(alias = "h2")]
#[non_exhaustive]
pub struct Heading2<T: crate::categories::PhrasingContent> {
    sys: html_sys::sections::Heading2,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading2<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading2<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading2<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::sections::Heading2> for Heading2<T> {
    fn into(self) -> html_sys::sections::Heading2 {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::sections::Heading2>
for Heading2<T> {
    fn from(sys: html_sys::sections::Heading2) -> Self {
        Self { sys, _children: vec![] }
    }
}
