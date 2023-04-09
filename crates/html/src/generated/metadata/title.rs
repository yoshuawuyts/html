pub mod element {
    /// The HTML `<title>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
    #[doc(alias = "title")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Title {
        sys: html_sys::metadata::Title,
        children: Vec<super::child::TitleChild>,
    }
    impl Title {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::TitleChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::TitleChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Title {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
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
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Title` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
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
    impl std::convert::From<crate::generated::all::Link> for TitleChild {
        fn from(value: crate::generated::all::Link) -> Self {
            Self::Link(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for TitleChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Style> for TitleChild {
        fn from(value: crate::generated::all::Style) -> Self {
            Self::Style(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for TitleChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::fmt::Display for TitleChild {
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
