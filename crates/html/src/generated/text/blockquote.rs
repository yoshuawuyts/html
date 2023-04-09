/// The HTML `<blockquote>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
#[doc(alias = "blockquote")]
#[non_exhaustive]
pub struct BlockQuote<T: crate::categories::FlowContent> {
    _sys: html_sys::text::BlockQuote,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent
for BlockQuote<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for BlockQuote<T> {}
