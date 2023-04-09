/// The HTML `<dd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
#[doc(alias = "dd")]
#[non_exhaustive]
pub struct DescriptionDetails<T: crate::categories::FlowContent> {
    _sys: html_sys::text::DescriptionDetails,
    _children: Vec<T>,
}
