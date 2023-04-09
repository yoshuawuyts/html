pub mod element {
    /// The HTML `<hgroup>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
    #[doc(alias = "hgroup")]
    #[non_exhaustive]
    pub struct HeadingGroup {
        sys: html_sys::sections::HeadingGroup,
        _children: Vec<super::child::HeadingGroupChild>,
    }
    impl crate::HtmlElement for HeadingGroup {}
    impl crate::FlowContent for HeadingGroup {}
    impl crate::HeadingContent for HeadingGroup {}
    impl crate::PalpableContent for HeadingGroup {}
    impl std::convert::Into<html_sys::sections::HeadingGroup> for HeadingGroup {
        fn into(self) -> html_sys::sections::HeadingGroup {
            self.sys
        }
    }
    impl From<html_sys::sections::HeadingGroup> for HeadingGroup {
        fn from(sys: html_sys::sections::HeadingGroup) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `HeadingGroup` element
    pub enum HeadingGroupChild {
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
