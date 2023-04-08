/// The HTML `<ul>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul)
#[doc(alias = "ul")]
#[non_exhaustive]
pub struct UnorderedList {
    _sys: html_sys::text::UnorderedList,
}
impl crate::categories::FlowContent for UnorderedList {}
