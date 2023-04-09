/// The HTML `<blockquote>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
#[doc(alias = "blockquote")]
#[non_exhaustive]
pub struct BlockQuote {
    sys: html_sys::text::BlockQuote,
    _children: Vec<()>,
}
impl BlockQuote {
    /// Get the value of the `cite` attribute
    pub fn cite(&self) -> std::option::Option<&str> {
        self.sys.cite.as_deref()
    }
    /// Set the value of the `cite` attribute
    pub fn set_cite(&mut self, value: std::option::Option<String>) {
        self.sys.cite = value;
    }
}
impl crate::HtmlElement for BlockQuote {}
impl crate::categories::FlowContent for BlockQuote {}
impl crate::categories::PalpableContent for BlockQuote {}
impl std::convert::Into<html_sys::text::BlockQuote> for BlockQuote {
    fn into(self) -> html_sys::text::BlockQuote {
        self.sys
    }
}
impl From<html_sys::text::BlockQuote> for BlockQuote {
    fn from(sys: html_sys::text::BlockQuote) -> Self {
        Self { sys, _children: vec![] }
    }
}
