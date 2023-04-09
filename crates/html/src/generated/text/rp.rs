/// The HTML `<rp>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rp)
#[doc(alias = "rp")]
#[non_exhaustive]
pub struct RubyFallbackParenthesis {
    sys: html_sys::text::RubyFallbackParenthesis,
}
impl crate::HtmlElement for RubyFallbackParenthesis {}
impl std::convert::Into<html_sys::text::RubyFallbackParenthesis>
for RubyFallbackParenthesis {
    fn into(self) -> html_sys::text::RubyFallbackParenthesis {
        self.sys
    }
}
impl From<html_sys::text::RubyFallbackParenthesis> for RubyFallbackParenthesis {
    fn from(sys: html_sys::text::RubyFallbackParenthesis) -> Self {
        Self { sys }
    }
}
