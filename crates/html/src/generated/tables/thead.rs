pub mod element {
    /// The HTML `<thead>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/thead)
    #[doc(alias = "thead")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableHead {
        sys: html_sys::tables::TableHead,
    }
    impl std::fmt::Display for TableHead {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableHead {}
    impl std::convert::Into<html_sys::tables::TableHead> for TableHead {
        fn into(self) -> html_sys::tables::TableHead {
            self.sys
        }
    }
    impl From<html_sys::tables::TableHead> for TableHead {
        fn from(sys: html_sys::tables::TableHead) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
