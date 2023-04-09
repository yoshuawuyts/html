pub mod element {
    /// The HTML `<colgroup>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
    #[doc(alias = "colgroup")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableColumnGroup {
        sys: html_sys::tables::TableColumnGroup,
    }
    impl TableColumnGroup {
        /// Get the value of the `span` attribute
        pub fn span(&self) -> std::option::Option<&str> {
            self.sys.span.as_deref()
        }
        /// Set the value of the `span` attribute
        pub fn set_span(&mut self, value: std::option::Option<String>) {
            self.sys.span = value;
        }
    }
    impl std::fmt::Display for TableColumnGroup {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableColumnGroup {}
    impl std::convert::Into<html_sys::tables::TableColumnGroup> for TableColumnGroup {
        fn into(self) -> html_sys::tables::TableColumnGroup {
            self.sys
        }
    }
    impl From<html_sys::tables::TableColumnGroup> for TableColumnGroup {
        fn from(sys: html_sys::tables::TableColumnGroup) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
