pub mod element {
    /// The HTML `<optgroup>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
    #[doc(alias = "optgroup")]
    #[non_exhaustive]
    pub struct OptionGroup {
        sys: html_sys::forms::OptionGroup,
    }
    impl OptionGroup {
        /// Get the value of the `disabled` attribute
        pub fn disabled(&self) -> bool {
            self.sys.disabled
        }
        /// Set the value of the `disabled` attribute
        pub fn set_disabled(&mut self, value: bool) {
            self.sys.disabled = value;
        }
        /// Get the value of the `label` attribute
        pub fn label(&self) -> std::option::Option<&str> {
            self.sys.label.as_deref()
        }
        /// Set the value of the `label` attribute
        pub fn set_label(&mut self, value: std::option::Option<String>) {
            self.sys.label = value;
        }
    }
    impl std::fmt::Display for OptionGroup {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for OptionGroup {}
    impl std::convert::Into<html_sys::forms::OptionGroup> for OptionGroup {
        fn into(self) -> html_sys::forms::OptionGroup {
            self.sys
        }
    }
    impl From<html_sys::forms::OptionGroup> for OptionGroup {
        fn from(sys: html_sys::forms::OptionGroup) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
