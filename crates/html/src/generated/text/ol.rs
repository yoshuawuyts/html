pub mod element {
    /// The HTML `<ol>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
    #[doc(alias = "ol")]
    #[non_exhaustive]
    pub struct OrderedList {
        sys: html_sys::text::OrderedList,
        _children: Vec<super::child::OrderedListChild>,
    }
    impl OrderedList {
        /// Get the value of the `reversed` attribute
        pub fn reversed(&self) -> std::option::Option<&str> {
            self.sys.reversed.as_deref()
        }
        /// Set the value of the `reversed` attribute
        pub fn set_reversed(&mut self, value: std::option::Option<String>) {
            self.sys.reversed = value;
        }
        /// Get the value of the `start` attribute
        pub fn start(&self) -> std::option::Option<&str> {
            self.sys.start.as_deref()
        }
        /// Set the value of the `start` attribute
        pub fn set_start(&mut self, value: std::option::Option<String>) {
            self.sys.start = value;
        }
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(&mut self, value: std::option::Option<String>) {
            self.sys.type_ = value;
        }
    }
    impl crate::HtmlElement for OrderedList {}
    impl crate::FlowContent for OrderedList {}
    impl std::convert::Into<html_sys::text::OrderedList> for OrderedList {
        fn into(self) -> html_sys::text::OrderedList {
            self.sys
        }
    }
    impl From<html_sys::text::OrderedList> for OrderedList {
        fn from(sys: html_sys::text::OrderedList) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `OrderedList` element
    pub enum OrderedListChild {
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
