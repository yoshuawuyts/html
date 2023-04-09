pub mod element {
    /// The HTML `<tfoot>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
    #[doc(alias = "tfoot")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableFoot {
        sys: html_sys::tables::TableFoot,
    }
    impl std::fmt::Display for TableFoot {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableFoot {}
    impl std::convert::Into<html_sys::tables::TableFoot> for TableFoot {
        fn into(self) -> html_sys::tables::TableFoot {
            self.sys
        }
    }
    impl From<html_sys::tables::TableFoot> for TableFoot {
        fn from(sys: html_sys::tables::TableFoot) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
