pub mod element {
    /// The HTML `<col>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
    #[doc(alias = "col")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableColumn {
        sys: html_sys::tables::TableColumn,
    }
    impl TableColumn {
        /// Get the value of the `span` attribute
        pub fn span(&self) -> std::option::Option<&str> {
            self.sys.span.as_deref()
        }
        /// Set the value of the `span` attribute
        pub fn set_span(&mut self, value: std::option::Option<String>) {
            self.sys.span = value;
        }
    }
    impl std::fmt::Display for TableColumn {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableColumn {}
    impl std::convert::Into<html_sys::tables::TableColumn> for TableColumn {
        fn into(self) -> html_sys::tables::TableColumn {
            self.sys
        }
    }
    impl From<html_sys::tables::TableColumn> for TableColumn {
        fn from(sys: html_sys::tables::TableColumn) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
