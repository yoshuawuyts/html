/// The HTML `<data>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
#[doc(alias = "data")]
#[non_exhaustive]
pub struct Data<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Data,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Data<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Data<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Data<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Data>
for Data<T> {
    fn into(self) -> html_sys::text::Data {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Data> for Data<T> {
    fn from(sys: html_sys::text::Data) -> Self {
        Self { sys, _children: vec![] }
    }
}
