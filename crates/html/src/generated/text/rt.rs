pub mod element {
    /// The HTML `<rt>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
    #[doc(alias = "rt")]
    #[non_exhaustive]
    pub struct RubyText {
        sys: html_sys::text::RubyText,
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
