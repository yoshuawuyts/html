/// The HTML `<em>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
#[doc(alias = "em")]
#[non_exhaustive]
pub struct Emphasis {
    sys: html_sys::text::Emphasis,
    _children: Vec<()>,
}
impl crate::HtmlElement for Emphasis {}
impl crate::categories::FlowContent for Emphasis {}
impl crate::categories::PhrasingContent for Emphasis {}
impl crate::categories::PalpableContent for Emphasis {}
impl std::convert::Into<html_sys::text::Emphasis> for Emphasis {
    fn into(self) -> html_sys::text::Emphasis {
        self.sys
    }
}
impl From<html_sys::text::Emphasis> for Emphasis {
    fn from(sys: html_sys::text::Emphasis) -> Self {
        Self { sys, _children: vec![] }
    }
}
