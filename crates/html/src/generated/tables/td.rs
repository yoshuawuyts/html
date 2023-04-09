pub mod element {
    /// The HTML `<td>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
    #[doc(alias = "td")]
    #[non_exhaustive]
    pub struct TableCell {
        sys: html_sys::tables::TableCell,
    }
    impl TableCell {
        /// Get the value of the `colspan` attribute
        pub fn colspan(&self) -> std::option::Option<&str> {
            self.sys.colspan.as_deref()
        }
        /// Set the value of the `colspan` attribute
        pub fn set_colspan(&mut self, value: std::option::Option<String>) {
            self.sys.colspan = value;
        }
        /// Get the value of the `rowspan` attribute
        pub fn rowspan(&self) -> std::option::Option<&str> {
            self.sys.rowspan.as_deref()
        }
        /// Set the value of the `rowspan` attribute
        pub fn set_rowspan(&mut self, value: std::option::Option<String>) {
            self.sys.rowspan = value;
        }
        /// Get the value of the `headers` attribute
        pub fn headers(&self) -> std::option::Option<&str> {
            self.sys.headers.as_deref()
        }
        /// Set the value of the `headers` attribute
        pub fn set_headers(&mut self, value: std::option::Option<String>) {
            self.sys.headers = value;
        }
    }
    impl std::fmt::Display for TableCell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableCell {}
    impl std::convert::Into<html_sys::tables::TableCell> for TableCell {
        fn into(self) -> html_sys::tables::TableCell {
            self.sys
        }
    }
    impl From<html_sys::tables::TableCell> for TableCell {
        fn from(sys: html_sys::tables::TableCell) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
