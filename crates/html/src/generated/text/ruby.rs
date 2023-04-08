/// The HTML `<ruby>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby)
#[doc(alias = "ruby")]
#[non_exhaustive]
pub struct RubyAnnotation {
    _sys: html_sys::text::RubyAnnotation,
}
impl crate::categories::FlowContent for RubyAnnotation {}
impl crate::categories::PhrasingContent for RubyAnnotation {}
impl crate::categories::PalpableContent for RubyAnnotation {}
