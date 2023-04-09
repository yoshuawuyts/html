pub mod element {
    /// The HTML `<nav>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
    #[doc(alias = "nav")]
    #[non_exhaustive]
    pub struct Navigation {
        sys: html_sys::sections::Navigation,
        _children: Vec<super::child::NavigationChild>,
    }
    impl crate::HtmlElement for Navigation {}
    impl crate::FlowContent for Navigation {}
    impl crate::SectioningContent for Navigation {}
    impl crate::PalpableContent for Navigation {}
    impl std::convert::Into<html_sys::sections::Navigation> for Navigation {
        fn into(self) -> html_sys::sections::Navigation {
            self.sys
        }
    }
    impl From<html_sys::sections::Navigation> for Navigation {
        fn from(sys: html_sys::sections::Navigation) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Navigation` element
    pub enum NavigationChild {
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
