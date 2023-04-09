pub mod element {
    /// The HTML `<source>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
    #[doc(alias = "source")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct MediaSource {
        sys: html_sys::embedded::MediaSource,
    }
    impl MediaSource {
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(&mut self, value: std::option::Option<String>) {
            self.sys.type_ = value;
        }
    }
    impl std::fmt::Display for MediaSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for MediaSource {}
    impl std::convert::Into<html_sys::embedded::MediaSource> for MediaSource {
        fn into(self) -> html_sys::embedded::MediaSource {
            self.sys
        }
    }
    impl From<html_sys::embedded::MediaSource> for MediaSource {
        fn from(sys: html_sys::embedded::MediaSource) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
