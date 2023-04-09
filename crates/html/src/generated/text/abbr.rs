/// The HTML `<abbr>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
#[doc(alias = "abbr")]
#[non_exhaustive]
pub struct Abbreviation<T: crate::categories::PhrasingContent> {
    _sys: html_sys::text::Abbreviation,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Abbreviation<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Abbreviation<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Abbreviation<T> {}
