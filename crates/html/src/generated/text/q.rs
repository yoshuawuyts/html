/// The HTML `<q>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q)
#[doc(alias = "q")]
#[non_exhaustive]
pub struct Quotation {
    _sys: html_sys::text::Quotation,
}
impl crate::categories::FlowContent for Quotation {}
impl crate::categories::PhrasingContent for Quotation {}
impl crate::categories::PalpableContent for Quotation {}
