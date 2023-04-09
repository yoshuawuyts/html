pub mod element {
    /// The HTML `<details>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
    #[doc(alias = "details")]
    #[non_exhaustive]
    pub struct Details {
        sys: html_sys::interactive::Details,
        _children: Vec<super::child::DetailsChild>,
    }
    impl Details {
        /// Get the value of the `open` attribute
        pub fn open(&self) -> bool {
            self.sys.open
        }
        /// Set the value of the `open` attribute
        pub fn set_open(&mut self, value: bool) {
            self.sys.open = value;
        }
    }
    impl crate::HtmlElement for Details {}
    impl crate::FlowContent for Details {}
    impl crate::InteractiveContent for Details {}
    impl crate::PalpableContent for Details {}
    impl std::convert::Into<html_sys::interactive::Details> for Details {
        fn into(self) -> html_sys::interactive::Details {
            self.sys
        }
    }
    impl From<html_sys::interactive::Details> for Details {
        fn from(sys: html_sys::interactive::Details) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Details` element
    pub enum DetailsChild {
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
}
