pub mod element {
    /// The HTML `<head>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head)
    #[doc(alias = "head")]
    #[non_exhaustive]
    pub struct Head {
        sys: html_sys::metadata::Head,
    }
    impl std::fmt::Display for Head {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Head {}
    impl std::convert::Into<html_sys::metadata::Head> for Head {
        fn into(self) -> html_sys::metadata::Head {
            self.sys
        }
    }
    impl From<html_sys::metadata::Head> for Head {
        fn from(sys: html_sys::metadata::Head) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
