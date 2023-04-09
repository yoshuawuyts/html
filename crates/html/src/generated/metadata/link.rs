pub mod element {
    /// The HTML `<link>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
    #[doc(alias = "link")]
    #[non_exhaustive]
    pub struct Link {
        sys: html_sys::metadata::Link,
        children: Vec<super::child::LinkChild>,
    }
    impl Link {
        /// Get the value of the `href` attribute
        pub fn href(&self) -> std::option::Option<&str> {
            self.sys.href.as_deref()
        }
        /// Set the value of the `href` attribute
        pub fn set_href(&mut self, value: std::option::Option<String>) {
            self.sys.href = value;
        }
        /// Get the value of the `crossorigin` attribute
        pub fn crossorigin(&self) -> std::option::Option<&str> {
            self.sys.crossorigin.as_deref()
        }
        /// Set the value of the `crossorigin` attribute
        pub fn set_crossorigin(&mut self, value: std::option::Option<String>) {
            self.sys.crossorigin = value;
        }
        /// Get the value of the `rel` attribute
        pub fn rel(&self) -> std::option::Option<&str> {
            self.sys.rel.as_deref()
        }
        /// Set the value of the `rel` attribute
        pub fn set_rel(&mut self, value: std::option::Option<String>) {
            self.sys.rel = value;
        }
        /// Get the value of the `media` attribute
        pub fn media(&self) -> std::option::Option<&str> {
            self.sys.media.as_deref()
        }
        /// Set the value of the `media` attribute
        pub fn set_media(&mut self, value: std::option::Option<String>) {
            self.sys.media = value;
        }
        /// Get the value of the `integrity` attribute
        pub fn integrity(&self) -> std::option::Option<&str> {
            self.sys.integrity.as_deref()
        }
        /// Set the value of the `integrity` attribute
        pub fn set_integrity(&mut self, value: std::option::Option<String>) {
            self.sys.integrity = value;
        }
        /// Get the value of the `hreflang` attribute
        pub fn hreflang(&self) -> std::option::Option<&str> {
            self.sys.hreflang.as_deref()
        }
        /// Set the value of the `hreflang` attribute
        pub fn set_hreflang(&mut self, value: std::option::Option<String>) {
            self.sys.hreflang = value;
        }
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(&mut self, value: std::option::Option<String>) {
            self.sys.type_ = value;
        }
        /// Get the value of the `referrerpolicy` attribute
        pub fn referrerpolicy(&self) -> std::option::Option<&str> {
            self.sys.referrerpolicy.as_deref()
        }
        /// Set the value of the `referrerpolicy` attribute
        pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
            self.sys.referrerpolicy = value;
        }
        /// Get the value of the `sizes` attribute
        pub fn sizes(&self) -> std::option::Option<&str> {
            self.sys.sizes.as_deref()
        }
        /// Set the value of the `sizes` attribute
        pub fn set_sizes(&mut self, value: std::option::Option<String>) {
            self.sys.sizes = value;
        }
        /// Get the value of the `imagesrcset` attribute
        pub fn imagesrcset(&self) -> std::option::Option<&str> {
            self.sys.imagesrcset.as_deref()
        }
        /// Set the value of the `imagesrcset` attribute
        pub fn set_imagesrcset(&mut self, value: std::option::Option<String>) {
            self.sys.imagesrcset = value;
        }
        /// Get the value of the `imagesizes` attribute
        pub fn imagesizes(&self) -> std::option::Option<&str> {
            self.sys.imagesizes.as_deref()
        }
        /// Set the value of the `imagesizes` attribute
        pub fn set_imagesizes(&mut self, value: std::option::Option<String>) {
            self.sys.imagesizes = value;
        }
        /// Get the value of the `as` attribute
        pub fn as_(&self) -> std::option::Option<&str> {
            self.sys.as_.as_deref()
        }
        /// Set the value of the `as` attribute
        pub fn set_as_(&mut self, value: std::option::Option<String>) {
            self.sys.as_ = value;
        }
        /// Get the value of the `blocking` attribute
        pub fn blocking(&self) -> std::option::Option<&str> {
            self.sys.blocking.as_deref()
        }
        /// Set the value of the `blocking` attribute
        pub fn set_blocking(&mut self, value: std::option::Option<String>) {
            self.sys.blocking = value;
        }
        /// Get the value of the `color` attribute
        pub fn color(&self) -> std::option::Option<&str> {
            self.sys.color.as_deref()
        }
        /// Set the value of the `color` attribute
        pub fn set_color(&mut self, value: std::option::Option<String>) {
            self.sys.color = value;
        }
        /// Get the value of the `disabled` attribute
        pub fn disabled(&self) -> std::option::Option<&str> {
            self.sys.disabled.as_deref()
        }
        /// Set the value of the `disabled` attribute
        pub fn set_disabled(&mut self, value: std::option::Option<String>) {
            self.sys.disabled = value;
        }
        /// Get the value of the `fetchpriority` attribute
        pub fn fetchpriority(&self) -> std::option::Option<&str> {
            self.sys.fetchpriority.as_deref()
        }
        /// Set the value of the `fetchpriority` attribute
        pub fn set_fetchpriority(&mut self, value: std::option::Option<String>) {
            self.sys.fetchpriority = value;
        }
    }
    impl Link {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::LinkChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::LinkChild> {
            &mut self.children
        }
    }
    impl crate::HtmlElement for Link {}
    impl crate::MetadataContent for Link {}
    impl std::convert::Into<html_sys::metadata::Link> for Link {
        fn into(self) -> html_sys::metadata::Link {
            self.sys
        }
    }
    impl From<html_sys::metadata::Link> for Link {
        fn from(sys: html_sys::metadata::Link) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Link` element
    pub enum LinkChild {
        /// The Link element
        Link(crate::generated::all::Link),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Style element
        Style(crate::generated::all::Style),
        /// The Template element
        Template(crate::generated::all::Template),
    }
    impl std::convert::From<crate::generated::all::Link> for LinkChild {
        fn from(value: crate::generated::all::Link) -> Self {
            Self::Link(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for LinkChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Style> for LinkChild {
        fn from(value: crate::generated::all::Style) -> Self {
            Self::Style(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for LinkChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
}
