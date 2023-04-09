/// The HTML `<b>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
#[doc(alias = "b")]
#[non_exhaustive]
pub struct Bold<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Bold,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Bold<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Bold<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Bold<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Bold>
for Bold<T> {
    fn into(self) -> html_sys::text::Bold {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Bold> for Bold<T> {
    fn from(sys: html_sys::text::Bold) -> Self {
        Self { sys, _children: vec![] }
    }
}
