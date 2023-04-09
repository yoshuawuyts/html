/// The HTML `<abbr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
#[doc(alias = "abbr")]
#[non_exhaustive]
pub struct Abbreviation {
    sys: html_sys::text::Abbreviation,
    _children: Vec<()>,
}
impl crate::HtmlElement for Abbreviation {}
impl crate::categories::FlowContent for Abbreviation {}
impl crate::categories::PhrasingContent for Abbreviation {}
impl crate::categories::PalpableContent for Abbreviation {}
impl std::convert::Into<html_sys::text::Abbreviation> for Abbreviation {
    fn into(self) -> html_sys::text::Abbreviation {
        self.sys
    }
}
impl From<html_sys::text::Abbreviation> for Abbreviation {
    fn from(sys: html_sys::text::Abbreviation) -> Self {
        Self { sys, _children: vec![] }
    }
}
