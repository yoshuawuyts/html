pub mod element {
    /// The HTML `<rp>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rp)
    #[doc(alias = "rp")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct RubyFallbackParenthesis {
        sys: html_sys::text::RubyFallbackParenthesis,
    }
    impl std::fmt::Display for RubyFallbackParenthesis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
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
}
pub mod child {}
