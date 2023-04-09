pub mod element {
    /// The HTML `<track>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
    #[doc(alias = "track")]
    #[non_exhaustive]
    pub struct TextTrack {
        sys: html_sys::embedded::TextTrack,
    }
    impl TextTrack {
        /// Get the value of the `kind` attribute
        pub fn kind(&self) -> std::option::Option<&str> {
            self.sys.kind.as_deref()
        }
        /// Set the value of the `kind` attribute
        pub fn set_kind(&mut self, value: std::option::Option<String>) {
            self.sys.kind = value;
        }
        /// Get the value of the `src` attribute
        pub fn src(&self) -> std::option::Option<&str> {
            self.sys.src.as_deref()
        }
        /// Set the value of the `src` attribute
        pub fn set_src(&mut self, value: std::option::Option<String>) {
            self.sys.src = value;
        }
        /// Get the value of the `srclang` attribute
        pub fn srclang(&self) -> std::option::Option<&str> {
            self.sys.srclang.as_deref()
        }
        /// Set the value of the `srclang` attribute
        pub fn set_srclang(&mut self, value: std::option::Option<String>) {
            self.sys.srclang = value;
        }
        /// Get the value of the `label` attribute
        pub fn label(&self) -> std::option::Option<&str> {
            self.sys.label.as_deref()
        }
        /// Set the value of the `label` attribute
        pub fn set_label(&mut self, value: std::option::Option<String>) {
            self.sys.label = value;
        }
        /// Get the value of the `default` attribute
        pub fn default(&self) -> bool {
            self.sys.default
        }
        /// Set the value of the `default` attribute
        pub fn set_default(&mut self, value: bool) {
            self.sys.default = value;
        }
    }
    impl std::fmt::Display for TextTrack {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TextTrack {}
    impl std::convert::Into<html_sys::embedded::TextTrack> for TextTrack {
        fn into(self) -> html_sys::embedded::TextTrack {
            self.sys
        }
    }
    impl From<html_sys::embedded::TextTrack> for TextTrack {
        fn from(sys: html_sys::embedded::TextTrack) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
