/// The HTML `<s>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
#[doc(alias = "s")]
#[non_exhaustive]
pub struct StrikeThrough<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::StrikeThrough,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for StrikeThrough<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for StrikeThrough<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for StrikeThrough<T> {}
impl<
    T: crate::categories::PhrasingContent,
> std::convert::Into<html_sys::text::StrikeThrough> for StrikeThrough<T> {
    fn into(self) -> html_sys::text::StrikeThrough {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::StrikeThrough>
for StrikeThrough<T> {
    fn from(sys: html_sys::text::StrikeThrough) -> Self {
        Self { sys, _children: vec![] }
    }
}
