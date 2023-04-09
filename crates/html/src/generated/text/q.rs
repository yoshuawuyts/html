/// The HTML `<q>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q)
#[doc(alias = "q")]
#[non_exhaustive]
pub struct Quotation<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Quotation,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Quotation<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Quotation<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Quotation<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Quotation>
for Quotation<T> {
    fn into(self) -> html_sys::text::Quotation {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Quotation>
for Quotation<T> {
    fn from(sys: html_sys::text::Quotation) -> Self {
        Self { sys, _children: vec![] }
    }
}
