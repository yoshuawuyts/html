/// The HTML `<i>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i)
#[doc(alias = "i")]
#[non_exhaustive]
pub struct Italic {
    sys: html_sys::text::Italic,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Italic {}
impl crate::categories::PhrasingContent for Italic {}
impl crate::categories::PalpableContent for Italic {}
impl std::convert::Into<html_sys::text::Italic> for Italic {
    fn into(self) -> html_sys::text::Italic {
        self.sys
    }
}
impl From<html_sys::text::Italic> for Italic {
    fn from(sys: html_sys::text::Italic) -> Self {
        Self { sys, _children: vec![] }
    }
}
