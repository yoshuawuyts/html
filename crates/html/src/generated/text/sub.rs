/// The HTML `<sub>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
#[doc(alias = "sub")]
#[non_exhaustive]
pub struct SubScript {
    sys: html_sys::text::SubScript,
    _children: Vec<()>,
}
impl crate::HtmlElement for SubScript {}
impl crate::categories::FlowContent for SubScript {}
impl crate::categories::PhrasingContent for SubScript {}
impl crate::categories::PalpableContent for SubScript {}
impl std::convert::Into<html_sys::text::SubScript> for SubScript {
    fn into(self) -> html_sys::text::SubScript {
        self.sys
    }
}
impl From<html_sys::text::SubScript> for SubScript {
    fn from(sys: html_sys::text::SubScript) -> Self {
        Self { sys, _children: vec![] }
    }
}
