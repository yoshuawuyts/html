/// The HTML `<blockquote>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
#[doc(alias = "blockquote")]
#[non_exhaustive]
pub struct BlockQuote {
    _sys: html_sys::text::BlockQuote,
}
impl crate::categories::FlowContent for BlockQuote {}
impl crate::categories::PalpableContent for BlockQuote {}
