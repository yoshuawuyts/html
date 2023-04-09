/// The HTML `<address>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address)
#[doc(alias = "address")]
#[non_exhaustive]
pub struct Address {
    sys: html_sys::sections::Address,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Address {}
impl crate::categories::PalpableContent for Address {}
impl std::convert::Into<html_sys::sections::Address> for Address {
    fn into(self) -> html_sys::sections::Address {
        self.sys
    }
}
impl From<html_sys::sections::Address> for Address {
    fn from(sys: html_sys::sections::Address) -> Self {
        Self { sys, _children: vec![] }
    }
}
