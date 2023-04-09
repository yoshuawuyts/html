pub mod element {
    /// The HTML `<colgroup>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
    #[doc(alias = "colgroup")]
    #[non_exhaustive]
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
