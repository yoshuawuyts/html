/// The HTML `<code>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
#[doc(alias = "code")]
#[non_exhaustive]
pub struct Code {
    _sys: html_sys::text::Code,
}
impl crate::categories::FlowContent for Code {}
impl crate::categories::PhrasingContent for Code {}
impl crate::categories::PalpableContent for Code {}
