pub mod element {
    /// The HTML `<h2>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
    #[doc(alias = "h2")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Heading2 {
        sys: html_sys::sections::Heading2,
        children: Vec<super::child::Heading2Child>,
    }
    impl Heading2 {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::Heading2Child] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::Heading2Child> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Heading2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Heading2 {}
    impl crate::FlowContent for Heading2 {}
    impl crate::HeadingContent for Heading2 {}
    impl crate::PalpableContent for Heading2 {}
    impl std::convert::Into<html_sys::sections::Heading2> for Heading2 {
        fn into(self) -> html_sys::sections::Heading2 {
            self.sys
        }
    }
    impl From<html_sys::sections::Heading2> for Heading2 {
        fn from(sys: html_sys::sections::Heading2) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Heading2` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub enum Heading2Child {
        /// The Address element
        Address(crate::generated::all::Address),
        /// The BlockQuote element
        BlockQuote(crate::generated::all::BlockQuote),
        /// The DescriptionList element
        DescriptionList(crate::generated::all::DescriptionList),
        /// The Details element
        Details(crate::generated::all::Details),
        /// The Dialog element
        Dialog(crate::generated::all::Dialog),
        /// The Division element
        Division(crate::generated::all::Division),
        /// The Fieldset element
        Fieldset(crate::generated::all::Fieldset),
        /// The Figure element
        Figure(crate::generated::all::Figure),
        /// The Footer element
        Footer(crate::generated::all::Footer),
        /// The Form element
        Form(crate::generated::all::Form),
        /// The Header element
        Header(crate::generated::all::Header),
        /// The Heading1 element
        Heading1(crate::generated::all::Heading1),
        /// The Heading2 element
        Heading2(crate::generated::all::Heading2),
        /// The Heading3 element
        Heading3(crate::generated::all::Heading3),
        /// The Heading4 element
        Heading4(crate::generated::all::Heading4),
        /// The Heading5 element
        Heading5(crate::generated::all::Heading5),
        /// The Heading6 element
        Heading6(crate::generated::all::Heading6),
        /// The HeadingGroup element
        HeadingGroup(crate::generated::all::HeadingGroup),
        /// The Menu element
        Menu(crate::generated::all::Menu),
        /// The OrderedList element
        OrderedList(crate::generated::all::OrderedList),
        /// The Paragraph element
        Paragraph(crate::generated::all::Paragraph),
        /// The PreformattedText element
        PreformattedText(crate::generated::all::PreformattedText),
        /// The Table element
        Table(crate::generated::all::Table),
        /// The ThematicBreak element
        ThematicBreak(crate::generated::all::ThematicBreak),
        /// The UnorderedList element
        UnorderedList(crate::generated::all::UnorderedList),
    }
    impl std::convert::From<crate::generated::all::Address> for Heading2Child {
        fn from(value: crate::generated::all::Address) -> Self {
            Self::Address(value)
        }
    }
    impl std::convert::From<crate::generated::all::BlockQuote> for Heading2Child {
        fn from(value: crate::generated::all::BlockQuote) -> Self {
            Self::BlockQuote(value)
        }
    }
    impl std::convert::From<crate::generated::all::DescriptionList> for Heading2Child {
        fn from(value: crate::generated::all::DescriptionList) -> Self {
            Self::DescriptionList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Details> for Heading2Child {
        fn from(value: crate::generated::all::Details) -> Self {
            Self::Details(value)
        }
    }
    impl std::convert::From<crate::generated::all::Dialog> for Heading2Child {
        fn from(value: crate::generated::all::Dialog) -> Self {
            Self::Dialog(value)
        }
    }
    impl std::convert::From<crate::generated::all::Division> for Heading2Child {
        fn from(value: crate::generated::all::Division) -> Self {
            Self::Division(value)
        }
    }
    impl std::convert::From<crate::generated::all::Fieldset> for Heading2Child {
        fn from(value: crate::generated::all::Fieldset) -> Self {
            Self::Fieldset(value)
        }
    }
    impl std::convert::From<crate::generated::all::Figure> for Heading2Child {
        fn from(value: crate::generated::all::Figure) -> Self {
            Self::Figure(value)
        }
    }
    impl std::convert::From<crate::generated::all::Footer> for Heading2Child {
        fn from(value: crate::generated::all::Footer) -> Self {
            Self::Footer(value)
        }
    }
    impl std::convert::From<crate::generated::all::Form> for Heading2Child {
        fn from(value: crate::generated::all::Form) -> Self {
            Self::Form(value)
        }
    }
    impl std::convert::From<crate::generated::all::Header> for Heading2Child {
        fn from(value: crate::generated::all::Header) -> Self {
            Self::Header(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading1> for Heading2Child {
        fn from(value: crate::generated::all::Heading1) -> Self {
            Self::Heading1(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading2> for Heading2Child {
        fn from(value: crate::generated::all::Heading2) -> Self {
            Self::Heading2(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading3> for Heading2Child {
        fn from(value: crate::generated::all::Heading3) -> Self {
            Self::Heading3(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading4> for Heading2Child {
        fn from(value: crate::generated::all::Heading4) -> Self {
            Self::Heading4(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading5> for Heading2Child {
        fn from(value: crate::generated::all::Heading5) -> Self {
            Self::Heading5(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading6> for Heading2Child {
        fn from(value: crate::generated::all::Heading6) -> Self {
            Self::Heading6(value)
        }
    }
    impl std::convert::From<crate::generated::all::HeadingGroup> for Heading2Child {
        fn from(value: crate::generated::all::HeadingGroup) -> Self {
            Self::HeadingGroup(value)
        }
    }
    impl std::convert::From<crate::generated::all::Menu> for Heading2Child {
        fn from(value: crate::generated::all::Menu) -> Self {
            Self::Menu(value)
        }
    }
    impl std::convert::From<crate::generated::all::OrderedList> for Heading2Child {
        fn from(value: crate::generated::all::OrderedList) -> Self {
            Self::OrderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Paragraph> for Heading2Child {
        fn from(value: crate::generated::all::Paragraph) -> Self {
            Self::Paragraph(value)
        }
    }
    impl std::convert::From<crate::generated::all::PreformattedText> for Heading2Child {
        fn from(value: crate::generated::all::PreformattedText) -> Self {
            Self::PreformattedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Table> for Heading2Child {
        fn from(value: crate::generated::all::Table) -> Self {
            Self::Table(value)
        }
    }
    impl std::convert::From<crate::generated::all::ThematicBreak> for Heading2Child {
        fn from(value: crate::generated::all::ThematicBreak) -> Self {
            Self::ThematicBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::UnorderedList> for Heading2Child {
        fn from(value: crate::generated::all::UnorderedList) -> Self {
            Self::UnorderedList(value)
        }
    }
    impl std::fmt::Display for Heading2Child {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Address(el) => write!(f, "{el}"),
                Self::BlockQuote(el) => write!(f, "{el}"),
                Self::DescriptionList(el) => write!(f, "{el}"),
                Self::Details(el) => write!(f, "{el}"),
                Self::Dialog(el) => write!(f, "{el}"),
                Self::Division(el) => write!(f, "{el}"),
                Self::Fieldset(el) => write!(f, "{el}"),
                Self::Figure(el) => write!(f, "{el}"),
                Self::Footer(el) => write!(f, "{el}"),
                Self::Form(el) => write!(f, "{el}"),
                Self::Header(el) => write!(f, "{el}"),
                Self::Heading1(el) => write!(f, "{el}"),
                Self::Heading2(el) => write!(f, "{el}"),
                Self::Heading3(el) => write!(f, "{el}"),
                Self::Heading4(el) => write!(f, "{el}"),
                Self::Heading5(el) => write!(f, "{el}"),
                Self::Heading6(el) => write!(f, "{el}"),
                Self::HeadingGroup(el) => write!(f, "{el}"),
                Self::Menu(el) => write!(f, "{el}"),
                Self::OrderedList(el) => write!(f, "{el}"),
                Self::Paragraph(el) => write!(f, "{el}"),
                Self::PreformattedText(el) => write!(f, "{el}"),
                Self::Table(el) => write!(f, "{el}"),
                Self::ThematicBreak(el) => write!(f, "{el}"),
                Self::UnorderedList(el) => write!(f, "{el}"),
            }
        }
    }
}
