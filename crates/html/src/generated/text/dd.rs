pub mod element {
    /// The HTML `<dd>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
    #[doc(alias = "dd")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct DescriptionDetails {
        sys: html_sys::text::DescriptionDetails,
    }
    impl std::fmt::Display for DescriptionDetails {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for DescriptionDetails {}
    impl std::convert::Into<html_sys::text::DescriptionDetails> for DescriptionDetails {
        fn into(self) -> html_sys::text::DescriptionDetails {
            self.sys
        }
    }
    impl From<html_sys::text::DescriptionDetails> for DescriptionDetails {
        fn from(sys: html_sys::text::DescriptionDetails) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
