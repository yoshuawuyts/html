/// The HTML `<u>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
#[doc(alias = "u")]
#[non_exhaustive]
pub struct Underline {
    sys: html_sys::text::Underline,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Underline {}
impl crate::categories::PhrasingContent for Underline {}
impl crate::categories::PalpableContent for Underline {}
impl std::convert::Into<html_sys::text::Underline> for Underline {
    fn into(self) -> html_sys::text::Underline {
        self.sys
    }
}
impl From<html_sys::text::Underline> for Underline {
    fn from(sys: html_sys::text::Underline) -> Self {
        Self { sys, _children: vec![] }
    }
}
