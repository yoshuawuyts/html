/// The HTML `<div>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
#[doc(alias = "div")]
#[non_exhaustive]
pub struct Division {
    sys: html_sys::text::Division,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Division {}
impl crate::categories::PalpableContent for Division {}
impl std::convert::Into<html_sys::text::Division> for Division {
    fn into(self) -> html_sys::text::Division {
        self.sys
    }
}
impl From<html_sys::text::Division> for Division {
    fn from(sys: html_sys::text::Division) -> Self {
        Self { sys, _children: vec![] }
    }
}
