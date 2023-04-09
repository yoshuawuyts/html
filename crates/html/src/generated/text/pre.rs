/// The HTML `<pre>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
#[doc(alias = "pre")]
#[non_exhaustive]
pub struct PreformattedText {
    sys: html_sys::text::PreformattedText,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for PreformattedText {}
impl crate::categories::PalpableContent for PreformattedText {}
impl std::convert::Into<html_sys::text::PreformattedText> for PreformattedText {
    fn into(self) -> html_sys::text::PreformattedText {
        self.sys
    }
}
impl From<html_sys::text::PreformattedText> for PreformattedText {
    fn from(sys: html_sys::text::PreformattedText) -> Self {
        Self { sys, _children: vec![] }
    }
}
