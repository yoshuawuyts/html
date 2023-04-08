/// The HTML `<address>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address)
#[doc(alias = "address")]
#[non_exhaustive]
pub struct Address {
    _sys: html_sys::sections::Address,
}
impl crate::categories::FlowContent for Address {}
impl crate::categories::PalpableContent for Address {}
