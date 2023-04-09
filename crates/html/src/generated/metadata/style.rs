pub mod element {
    /// The HTML `<style>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
    #[doc(alias = "style")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Style {
        sys: html_sys::metadata::Style,
        children: Vec<super::child::StyleChild>,
    }
    impl Style {
        /// Get the value of the `media` attribute
        pub fn media(&self) -> std::option::Option<&str> {
            self.sys.media.as_deref()
        }
        /// Set the value of the `media` attribute
        pub fn set_media(&mut self, value: std::option::Option<String>) {
            self.sys.media = value;
        }
        /// Get the value of the `blocking` attribute
        pub fn blocking(&self) -> std::option::Option<&str> {
            self.sys.blocking.as_deref()
        }
        /// Set the value of the `blocking` attribute
        pub fn set_blocking(&mut self, value: std::option::Option<String>) {
            self.sys.blocking = value;
        }
    }
    impl Style {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::StyleChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::StyleChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Style {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Style {}
    impl crate::MetadataContent for Style {}
    impl std::convert::Into<html_sys::metadata::Style> for Style {
        fn into(self) -> html_sys::metadata::Style {
            self.sys
        }
    }
    impl From<html_sys::metadata::Style> for Style {
        fn from(sys: html_sys::metadata::Style) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Style` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub enum StyleChild {
        /// The Link element
        Link(crate::generated::all::Link),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Style element
        Style(crate::generated::all::Style),
        /// The Template element
        Template(crate::generated::all::Template),
    }
    impl std::convert::From<crate::generated::all::Link> for StyleChild {
        fn from(value: crate::generated::all::Link) -> Self {
            Self::Link(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for StyleChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Style> for StyleChild {
        fn from(value: crate::generated::all::Style) -> Self {
            Self::Style(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for StyleChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::fmt::Display for StyleChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Link(el) => write!(f, "{el}"),
                Self::Script(el) => write!(f, "{el}"),
                Self::Style(el) => write!(f, "{el}"),
                Self::Template(el) => write!(f, "{el}"),
            }
        }
    }
}
