/// The HTML `<rt>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
#[doc(alias = "rt")]
#[non_exhaustive]
pub struct RubyText<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::RubyText,
    _children: Vec<T>,
}
