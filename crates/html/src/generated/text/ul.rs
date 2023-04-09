/// The HTML `<ul>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul)
#[doc(alias = "ul")]
#[non_exhaustive]
pub struct UnorderedList {
    sys: html_sys::text::UnorderedList,
    _children: Vec<()>,
}
impl crate::HtmlElement for UnorderedList {}
impl crate::categories::FlowContent for UnorderedList {}
impl std::convert::Into<html_sys::text::UnorderedList> for UnorderedList {
    fn into(self) -> html_sys::text::UnorderedList {
        self.sys
    }
}
impl From<html_sys::text::UnorderedList> for UnorderedList {
    fn from(sys: html_sys::text::UnorderedList) -> Self {
        Self { sys, _children: vec![] }
    }
}
