pub mod element {
    /// The HTML `<th>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
    #[doc(alias = "th")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableHeader {
        sys: html_sys::tables::TableHeader,
    }
    impl TableHeader {
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
        /// Get the value of the `scope` attribute
        pub fn scope(&self) -> std::option::Option<&str> {
            self.sys.scope.as_deref()
        }
        /// Set the value of the `scope` attribute
        pub fn set_scope(&mut self, value: std::option::Option<String>) {
            self.sys.scope = value;
        }
        /// Get the value of the `abbr` attribute
        pub fn abbr(&self) -> std::option::Option<&str> {
            self.sys.abbr.as_deref()
        }
        /// Set the value of the `abbr` attribute
        pub fn set_abbr(&mut self, value: std::option::Option<String>) {
            self.sys.abbr = value;
        }
    }
    impl std::fmt::Display for TableHeader {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableHeader {}
    impl std::convert::Into<html_sys::tables::TableHeader> for TableHeader {
        fn into(self) -> html_sys::tables::TableHeader {
            self.sys
        }
    }
    impl From<html_sys::tables::TableHeader> for TableHeader {
        fn from(sys: html_sys::tables::TableHeader) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
