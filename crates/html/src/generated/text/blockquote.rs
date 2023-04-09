pub mod element {
    /// The HTML `<blockquote>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
    #[doc(alias = "blockquote")]
    #[non_exhaustive]
    pub struct BlockQuote {
        sys: html_sys::text::BlockQuote,
        _children: Vec<super::child::BlockQuoteChild>,
    }
    impl BlockQuote {
        /// Get the value of the `cite` attribute
        pub fn cite(&self) -> std::option::Option<&str> {
            self.sys.cite.as_deref()
        }
        /// Set the value of the `cite` attribute
        pub fn set_cite(&mut self, value: std::option::Option<String>) {
            self.sys.cite = value;
        }
    }
    impl crate::HtmlElement for BlockQuote {}
    impl crate::FlowContent for BlockQuote {}
    impl crate::PalpableContent for BlockQuote {}
    impl std::convert::Into<html_sys::text::BlockQuote> for BlockQuote {
        fn into(self) -> html_sys::text::BlockQuote {
            self.sys
        }
    }
    impl From<html_sys::text::BlockQuote> for BlockQuote {
        fn from(sys: html_sys::text::BlockQuote) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `BlockQuote` element
    pub enum BlockQuoteChild {
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
