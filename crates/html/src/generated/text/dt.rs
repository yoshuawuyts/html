pub mod element {
    /// The HTML `<dt>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt)
    #[doc(alias = "dt")]
    #[non_exhaustive]
    pub struct DescriptionTerm {
        sys: html_sys::text::DescriptionTerm,
    }
    impl std::fmt::Display for DescriptionTerm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for DescriptionTerm {}
    impl std::convert::Into<html_sys::text::DescriptionTerm> for DescriptionTerm {
        fn into(self) -> html_sys::text::DescriptionTerm {
            self.sys
        }
    }
    impl From<html_sys::text::DescriptionTerm> for DescriptionTerm {
        fn from(sys: html_sys::text::DescriptionTerm) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
