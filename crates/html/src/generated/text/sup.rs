/// The HTML `<sup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
#[doc(alias = "sup")]
#[non_exhaustive]
pub struct SuperScript<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::SuperScript,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for SuperScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for SuperScript<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for SuperScript<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::SuperScript> for SuperScript<T> {
    fn into(self) -> html_sys::text::SuperScript {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::SuperScript>
for SuperScript<T> {
    fn from(sys: html_sys::text::SuperScript) -> Self {
        Self { sys, _children: vec![] }
    }
}
