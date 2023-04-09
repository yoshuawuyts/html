/// The HTML `<p>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
#[doc(alias = "p")]
#[non_exhaustive]
pub struct Paragraph {
    sys: html_sys::text::Paragraph,
    _children: Vec<()>,
}
impl crate::HtmlElement for Paragraph {}
impl crate::categories::FlowContent for Paragraph {}
impl crate::categories::PalpableContent for Paragraph {}
impl std::convert::Into<html_sys::text::Paragraph> for Paragraph {
    fn into(self) -> html_sys::text::Paragraph {
        self.sys
    }
}
impl From<html_sys::text::Paragraph> for Paragraph {
    fn from(sys: html_sys::text::Paragraph) -> Self {
        Self { sys, _children: vec![] }
    }
}
