pub mod element {
    /// The HTML `<tr>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr)
    #[doc(alias = "tr")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableRow {
        sys: html_sys::tables::TableRow,
    }
    impl std::fmt::Display for TableRow {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableRow {}
    impl std::convert::Into<html_sys::tables::TableRow> for TableRow {
        fn into(self) -> html_sys::tables::TableRow {
            self.sys
        }
    }
    impl From<html_sys::tables::TableRow> for TableRow {
        fn from(sys: html_sys::tables::TableRow) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
