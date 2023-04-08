/// The HTML `<b>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
#[doc(alias = "b")]
#[non_exhaustive]
pub struct Bold {
    _sys: html_sys::text::Bold,
}
impl crate::categories::FlowContent for Bold {}
impl crate::categories::PhrasingContent for Bold {}
impl crate::categories::PalpableContent for Bold {}
