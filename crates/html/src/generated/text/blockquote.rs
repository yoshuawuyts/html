/// The HTML `<blockquote>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
#[doc(alias = "blockquote")]
#[non_exhaustive]
pub struct BlockQuote<T: crate::categories::FlowContent> {
    sys: html_sys::text::BlockQuote,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> crate::categories::FlowContent
for BlockQuote<T> {}
impl<T: crate::categories::FlowContent> crate::categories::PalpableContent
for BlockQuote<T> {}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::text::BlockQuote>
for BlockQuote<T> {
    fn into(self) -> html_sys::text::BlockQuote {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::text::BlockQuote>
for BlockQuote<T> {
    fn from(sys: html_sys::text::BlockQuote) -> Self {
        Self { sys, _children: vec![] }
    }
}
