pub mod element {
    /// The HTML `<rt>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
    #[doc(alias = "rt")]
    #[non_exhaustive]
    pub struct RubyText {
        sys: html_sys::text::RubyText,
    }
    impl std::fmt::Display for RubyText {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for RubyText {}
    impl std::convert::Into<html_sys::text::RubyText> for RubyText {
        fn into(self) -> html_sys::text::RubyText {
            self.sys
        }
    }
    impl From<html_sys::text::RubyText> for RubyText {
        fn from(sys: html_sys::text::RubyText) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
