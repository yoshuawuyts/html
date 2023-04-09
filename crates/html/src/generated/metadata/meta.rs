pub mod element {
    /// The HTML `<meta>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
    #[doc(alias = "meta")]
    #[non_exhaustive]
    pub struct Meta {
        sys: html_sys::metadata::Meta,
        children: Vec<super::child::MetaChild>,
    }
    impl Meta {
        /// Get the value of the `name` attribute
        pub fn name(&self) -> std::option::Option<&str> {
            self.sys.name.as_deref()
        }
        /// Set the value of the `name` attribute
        pub fn set_name(&mut self, value: std::option::Option<String>) {
            self.sys.name = value;
        }
        /// Get the value of the `http-equiv` attribute
        pub fn http_equiv(&self) -> std::option::Option<&str> {
            self.sys.http_equiv.as_deref()
        }
        /// Set the value of the `http-equiv` attribute
        pub fn set_http_equiv(&mut self, value: std::option::Option<String>) {
            self.sys.http_equiv = value;
        }
        /// Get the value of the `content` attribute
        pub fn content(&self) -> std::option::Option<&str> {
            self.sys.content.as_deref()
        }
        /// Set the value of the `content` attribute
        pub fn set_content(&mut self, value: std::option::Option<String>) {
            self.sys.content = value;
        }
        /// Get the value of the `charset` attribute
        pub fn charset(&self) -> std::option::Option<&str> {
            self.sys.charset.as_deref()
        }
        /// Set the value of the `charset` attribute
        pub fn set_charset(&mut self, value: std::option::Option<String>) {
            self.sys.charset = value;
        }
        /// Get the value of the `media` attribute
        pub fn media(&self) -> std::option::Option<&str> {
            self.sys.media.as_deref()
        }
        /// Set the value of the `media` attribute
        pub fn set_media(&mut self, value: std::option::Option<String>) {
            self.sys.media = value;
        }
    }
    impl Meta {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::MetaChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::MetaChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Meta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Meta {}
    impl crate::MetadataContent for Meta {}
    impl std::convert::Into<html_sys::metadata::Meta> for Meta {
        fn into(self) -> html_sys::metadata::Meta {
            self.sys
        }
    }
    impl From<html_sys::metadata::Meta> for Meta {
        fn from(sys: html_sys::metadata::Meta) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Meta` element
    pub enum MetaChild {
        /// The Link element
        Link(crate::generated::all::Link),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Style element
        Style(crate::generated::all::Style),
        /// The Template element
        Template(crate::generated::all::Template),
    }
    impl std::convert::From<crate::generated::all::Link> for MetaChild {
        fn from(value: crate::generated::all::Link) -> Self {
            Self::Link(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for MetaChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Style> for MetaChild {
        fn from(value: crate::generated::all::Style) -> Self {
            Self::Style(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for MetaChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::fmt::Display for MetaChild {
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
