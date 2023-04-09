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
}
