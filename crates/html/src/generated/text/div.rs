pub mod element {
    /// The HTML `<div>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    #[doc(alias = "div")]
    #[non_exhaustive]
    pub struct Division {
        sys: html_sys::text::Division,
        _children: Vec<super::child::DivisionChild>,
    }
    impl crate::HtmlElement for Division {}
    impl crate::FlowContent for Division {}
    impl crate::PalpableContent for Division {}
    impl std::convert::Into<html_sys::text::Division> for Division {
        fn into(self) -> html_sys::text::Division {
            self.sys
        }
    }
    impl From<html_sys::text::Division> for Division {
        fn from(sys: html_sys::text::Division) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Division` element
    pub enum DivisionChild {
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
    impl std::convert::From<crate::generated::all::Address> for DivisionChild {
        fn from(value: crate::generated::all::Address) -> Self {
            Self::Address(value)
        }
    }
    impl std::convert::From<crate::generated::all::BlockQuote> for DivisionChild {
        fn from(value: crate::generated::all::BlockQuote) -> Self {
            Self::BlockQuote(value)
        }
    }
    impl std::convert::From<crate::generated::all::DescriptionList> for DivisionChild {
        fn from(value: crate::generated::all::DescriptionList) -> Self {
            Self::DescriptionList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Details> for DivisionChild {
        fn from(value: crate::generated::all::Details) -> Self {
            Self::Details(value)
        }
    }
    impl std::convert::From<crate::generated::all::Dialog> for DivisionChild {
        fn from(value: crate::generated::all::Dialog) -> Self {
            Self::Dialog(value)
        }
    }
    impl std::convert::From<crate::generated::all::Division> for DivisionChild {
        fn from(value: crate::generated::all::Division) -> Self {
            Self::Division(value)
        }
    }
    impl std::convert::From<crate::generated::all::Fieldset> for DivisionChild {
        fn from(value: crate::generated::all::Fieldset) -> Self {
            Self::Fieldset(value)
        }
    }
    impl std::convert::From<crate::generated::all::Figure> for DivisionChild {
        fn from(value: crate::generated::all::Figure) -> Self {
            Self::Figure(value)
        }
    }
    impl std::convert::From<crate::generated::all::Footer> for DivisionChild {
        fn from(value: crate::generated::all::Footer) -> Self {
            Self::Footer(value)
        }
    }
    impl std::convert::From<crate::generated::all::Form> for DivisionChild {
        fn from(value: crate::generated::all::Form) -> Self {
            Self::Form(value)
        }
    }
    impl std::convert::From<crate::generated::all::Header> for DivisionChild {
        fn from(value: crate::generated::all::Header) -> Self {
            Self::Header(value)
        }
    }
    impl std::convert::From<crate::generated::all::Menu> for DivisionChild {
        fn from(value: crate::generated::all::Menu) -> Self {
            Self::Menu(value)
        }
    }
    impl std::convert::From<crate::generated::all::OrderedList> for DivisionChild {
        fn from(value: crate::generated::all::OrderedList) -> Self {
            Self::OrderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Paragraph> for DivisionChild {
        fn from(value: crate::generated::all::Paragraph) -> Self {
            Self::Paragraph(value)
        }
    }
    impl std::convert::From<crate::generated::all::PreformattedText> for DivisionChild {
        fn from(value: crate::generated::all::PreformattedText) -> Self {
            Self::PreformattedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Table> for DivisionChild {
        fn from(value: crate::generated::all::Table) -> Self {
            Self::Table(value)
        }
    }
    impl std::convert::From<crate::generated::all::ThematicBreak> for DivisionChild {
        fn from(value: crate::generated::all::ThematicBreak) -> Self {
            Self::ThematicBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::UnorderedList> for DivisionChild {
        fn from(value: crate::generated::all::UnorderedList) -> Self {
            Self::UnorderedList(value)
        }
    }
}
