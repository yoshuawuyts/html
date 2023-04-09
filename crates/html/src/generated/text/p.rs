pub mod element {
    /// The HTML `<p>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
    #[doc(alias = "p")]
    #[non_exhaustive]
    pub struct Paragraph {
        sys: html_sys::text::Paragraph,
        _children: Vec<super::child::ParagraphChild>,
    }
    impl crate::HtmlElement for Paragraph {}
    impl crate::FlowContent for Paragraph {}
    impl crate::PalpableContent for Paragraph {}
    impl std::convert::Into<html_sys::text::Paragraph> for Paragraph {
        fn into(self) -> html_sys::text::Paragraph {
            self.sys
        }
    }
    impl From<html_sys::text::Paragraph> for Paragraph {
        fn from(sys: html_sys::text::Paragraph) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Paragraph` element
    pub enum ParagraphChild {
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
