pub mod element {
    /// The HTML `<title>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
    #[doc(alias = "title")]
    #[non_exhaustive]
    pub struct Title {
        sys: html_sys::metadata::Title,
        _children: Vec<super::child::TitleChild>,
    }
    impl crate::HtmlElement for Title {}
    impl crate::MetadataContent for Title {}
    impl std::convert::Into<html_sys::metadata::Title> for Title {
        fn into(self) -> html_sys::metadata::Title {
            self.sys
        }
    }
    impl From<html_sys::metadata::Title> for Title {
        fn from(sys: html_sys::metadata::Title) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Title` element
    pub enum TitleChild {
        /// The Link element
        Link(crate::generated::all::Link),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Style element
        Style(crate::generated::all::Style),
        /// The Template element
        Template(crate::generated::all::Template),
    }
}
