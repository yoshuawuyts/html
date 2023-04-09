pub mod element {
    /// The HTML `<base>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
    #[doc(alias = "base")]
    #[non_exhaustive]
    pub struct Base {
        sys: html_sys::metadata::Base,
        children: Vec<super::child::BaseChild>,
    }
    impl Base {
        /// Get the value of the `href` attribute
        pub fn href(&self) -> std::option::Option<&str> {
            self.sys.href.as_deref()
        }
        /// Set the value of the `href` attribute
        pub fn set_href(&mut self, value: std::option::Option<String>) {
            self.sys.href = value;
        }
        /// Get the value of the `target` attribute
        pub fn target(&self) -> std::option::Option<&str> {
            self.sys.target.as_deref()
        }
        /// Set the value of the `target` attribute
        pub fn set_target(&mut self, value: std::option::Option<String>) {
            self.sys.target = value;
        }
    }
    impl Base {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::BaseChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::BaseChild> {
            &mut self.children
        }
    }
    impl crate::HtmlElement for Base {}
    impl crate::MetadataContent for Base {}
    impl std::convert::Into<html_sys::metadata::Base> for Base {
        fn into(self) -> html_sys::metadata::Base {
            self.sys
        }
    }
    impl From<html_sys::metadata::Base> for Base {
        fn from(sys: html_sys::metadata::Base) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Base` element
    pub enum BaseChild {
        /// The Link element
        Link(crate::generated::all::Link),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Style element
        Style(crate::generated::all::Style),
        /// The Template element
        Template(crate::generated::all::Template),
    }
    impl std::convert::From<crate::generated::all::Link> for BaseChild {
        fn from(value: crate::generated::all::Link) -> Self {
            Self::Link(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for BaseChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Style> for BaseChild {
        fn from(value: crate::generated::all::Style) -> Self {
            Self::Style(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for BaseChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
}
