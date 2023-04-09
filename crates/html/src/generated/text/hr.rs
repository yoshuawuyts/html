/// The HTML `<hr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr)
#[doc(alias = "hr")]
#[non_exhaustive]
pub struct ThematicBreak {
    sys: html_sys::text::ThematicBreak,
}
impl crate::categories::FlowContent for ThematicBreak {}
impl std::convert::Into<html_sys::text::ThematicBreak> for ThematicBreak {
    fn into(self) -> html_sys::text::ThematicBreak {
        self.sys
    }
}
impl From<html_sys::text::ThematicBreak> for ThematicBreak {
    fn from(sys: html_sys::text::ThematicBreak) -> Self {
        Self { sys }
    }
}
