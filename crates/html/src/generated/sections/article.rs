pub mod element {
    /// The HTML `<article>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
    #[doc(alias = "article")]
    #[non_exhaustive]
    pub struct Article {
        sys: html_sys::sections::Article,
        _children: Vec<super::child::ArticleChild>,
    }
    impl crate::HtmlElement for Article {}
    impl crate::FlowContent for Article {}
    impl crate::SectioningContent for Article {}
    impl crate::PalpableContent for Article {}
    impl std::convert::Into<html_sys::sections::Article> for Article {
        fn into(self) -> html_sys::sections::Article {
            self.sys
        }
    }
    impl From<html_sys::sections::Article> for Article {
        fn from(sys: html_sys::sections::Article) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Article` element
    pub enum ArticleChild {
        /// The Address element
        Address(crate::generated::all::Address),
        /// The Article element
        Article(crate::generated::all::Article),
        /// The Aside element
        Aside(crate::generated::all::Aside),
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
        /// The Navigation element
        Navigation(crate::generated::all::Navigation),
        /// The OrderedList element
        OrderedList(crate::generated::all::OrderedList),
        /// The Paragraph element
        Paragraph(crate::generated::all::Paragraph),
        /// The PreformattedText element
        PreformattedText(crate::generated::all::PreformattedText),
        /// The Section element
        Section(crate::generated::all::Section),
        /// The Table element
        Table(crate::generated::all::Table),
        /// The ThematicBreak element
        ThematicBreak(crate::generated::all::ThematicBreak),
        /// The UnorderedList element
        UnorderedList(crate::generated::all::UnorderedList),
    }
}
