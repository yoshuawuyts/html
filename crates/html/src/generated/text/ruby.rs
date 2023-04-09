/// The HTML `<ruby>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby)
#[doc(alias = "ruby")]
#[non_exhaustive]
pub struct RubyAnnotation {
    sys: html_sys::text::RubyAnnotation,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for RubyAnnotation {}
impl crate::categories::PhrasingContent for RubyAnnotation {}
impl crate::categories::PalpableContent for RubyAnnotation {}
impl std::convert::Into<html_sys::text::RubyAnnotation> for RubyAnnotation {
    fn into(self) -> html_sys::text::RubyAnnotation {
        self.sys
    }
}
impl From<html_sys::text::RubyAnnotation> for RubyAnnotation {
    fn from(sys: html_sys::text::RubyAnnotation) -> Self {
        Self { sys, _children: vec![] }
    }
}
