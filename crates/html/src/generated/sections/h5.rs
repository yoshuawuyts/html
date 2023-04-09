pub mod element {
    /// The HTML `<h5>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
    #[doc(alias = "h5")]
    #[non_exhaustive]
    pub struct Heading5 {
        sys: html_sys::sections::Heading5,
        _children: Vec<super::child::Heading5Child>,
    }
    impl crate::HtmlElement for Heading5 {}
    impl crate::FlowContent for Heading5 {}
    impl crate::HeadingContent for Heading5 {}
    impl crate::PalpableContent for Heading5 {}
    impl std::convert::Into<html_sys::sections::Heading5> for Heading5 {
        fn into(self) -> html_sys::sections::Heading5 {
            self.sys
        }
    }
    impl From<html_sys::sections::Heading5> for Heading5 {
        fn from(sys: html_sys::sections::Heading5) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Heading5` element
    pub enum Heading5Child {
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
}
