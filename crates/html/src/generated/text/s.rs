/// The HTML `<s>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
#[doc(alias = "s")]
#[non_exhaustive]
pub struct StrikeThrough {
    sys: html_sys::text::StrikeThrough,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for StrikeThrough {}
impl crate::categories::PhrasingContent for StrikeThrough {}
impl crate::categories::PalpableContent for StrikeThrough {}
impl std::convert::Into<html_sys::text::StrikeThrough> for StrikeThrough {
    fn into(self) -> html_sys::text::StrikeThrough {
        self.sys
    }
}
impl From<html_sys::text::StrikeThrough> for StrikeThrough {
    fn from(sys: html_sys::text::StrikeThrough) -> Self {
        Self { sys, _children: vec![] }
    }
}
