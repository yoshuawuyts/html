/// The HTML `<h5>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
#[doc(alias = "h5")]
#[non_exhaustive]
pub struct Heading5<T: crate::categories::PhrasingContent> {
    sys: html_sys::sections::Heading5,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Heading5<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::HeadingContent
for Heading5<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Heading5<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::sections::Heading5> for Heading5<T> {
    fn into(self) -> html_sys::sections::Heading5 {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::sections::Heading5>
for Heading5<T> {
    fn from(sys: html_sys::sections::Heading5) -> Self {
        Self { sys, _children: vec![] }
    }
}
