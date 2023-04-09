pub mod element {
    /// The HTML `<figcaption>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
    #[doc(alias = "figcaption")]
    #[non_exhaustive]
    pub struct FigureCaption {
        sys: html_sys::text::FigureCaption,
    }
    impl std::fmt::Display for FigureCaption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for FigureCaption {}
    impl std::convert::Into<html_sys::text::FigureCaption> for FigureCaption {
        fn into(self) -> html_sys::text::FigureCaption {
            self.sys
        }
    }
    impl From<html_sys::text::FigureCaption> for FigureCaption {
        fn from(sys: html_sys::text::FigureCaption) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
