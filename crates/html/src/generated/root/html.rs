pub mod element {
    /// The HTML `<html>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html)
    #[doc(alias = "html")]
    #[non_exhaustive]
    pub struct Html {
        sys: html_sys::root::Html,
    }
    impl std::fmt::Display for Html {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Html {}
    impl std::convert::Into<html_sys::root::Html> for Html {
        fn into(self) -> html_sys::root::Html {
            self.sys
        }
    }
    impl From<html_sys::root::Html> for Html {
        fn from(sys: html_sys::root::Html) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
