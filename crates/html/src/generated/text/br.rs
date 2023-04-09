/// The HTML `<br>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br)
#[doc(alias = "br")]
#[non_exhaustive]
pub struct LineBreak {
    sys: html_sys::text::LineBreak,
    _children: Vec<()>,
}
impl crate::HtmlElement for LineBreak {}
impl crate::categories::FlowContent for LineBreak {}
impl crate::categories::PhrasingContent for LineBreak {}
impl std::convert::Into<html_sys::text::LineBreak> for LineBreak {
    fn into(self) -> html_sys::text::LineBreak {
        self.sys
    }
}
impl From<html_sys::text::LineBreak> for LineBreak {
    fn from(sys: html_sys::text::LineBreak) -> Self {
        Self { sys, _children: vec![] }
    }
}
