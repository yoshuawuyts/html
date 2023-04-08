/// The HTML `<hr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr)
#[doc(alias = "hr")]
#[non_exhaustive]
pub struct ThematicBreak {
    _sys: html_sys::text::ThematicBreak,
}
impl crate::categories::FlowContent for ThematicBreak {}
