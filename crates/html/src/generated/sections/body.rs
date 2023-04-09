pub mod element {
    /// The HTML `<body>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
    #[doc(alias = "body")]
    #[non_exhaustive]
    pub struct Body {
        sys: html_sys::sections::Body,
    }
    impl std::fmt::Display for Body {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Body {}
    impl std::convert::Into<html_sys::sections::Body> for Body {
        fn into(self) -> html_sys::sections::Body {
            self.sys
        }
    }
    impl From<html_sys::sections::Body> for Body {
        fn from(sys: html_sys::sections::Body) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
