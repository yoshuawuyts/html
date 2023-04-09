pub mod element {
    /// The HTML `<caption>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/caption)
    #[doc(alias = "caption")]
    #[non_exhaustive]
    pub struct Caption {
        sys: html_sys::tables::Caption,
    }
    impl std::fmt::Display for Caption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Caption {}
    impl std::convert::Into<html_sys::tables::Caption> for Caption {
        fn into(self) -> html_sys::tables::Caption {
            self.sys
        }
    }
    impl From<html_sys::tables::Caption> for Caption {
        fn from(sys: html_sys::tables::Caption) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
