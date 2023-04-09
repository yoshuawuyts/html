pub mod element {
    /// The HTML `<address>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address)
    #[doc(alias = "address")]
    #[non_exhaustive]
    pub struct Address {
        sys: html_sys::sections::Address,
        _children: Vec<super::child::AddressChild>,
    }
    impl crate::HtmlElement for Address {}
    impl crate::FlowContent for Address {}
    impl crate::PalpableContent for Address {}
    impl std::convert::Into<html_sys::sections::Address> for Address {
        fn into(self) -> html_sys::sections::Address {
            self.sys
        }
    }
    impl From<html_sys::sections::Address> for Address {
        fn from(sys: html_sys::sections::Address) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Address` element
    pub enum AddressChild {
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
