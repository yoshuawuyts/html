/// The HTML `<rt>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
#[doc(alias = "rt")]
#[non_exhaustive]
pub struct RubyText<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::RubyText,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::RubyText>
for RubyText<T> {
    fn into(self) -> html_sys::text::RubyText {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::RubyText>
for RubyText<T> {
    fn from(sys: html_sys::text::RubyText) -> Self {
        Self { sys, _children: vec![] }
    }
}
