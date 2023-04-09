pub mod element {
    /// The HTML `<fieldset>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
    #[doc(alias = "fieldset")]
    #[non_exhaustive]
    pub struct Fieldset {
        sys: html_sys::forms::Fieldset,
        _children: Vec<super::child::FieldsetChild>,
    }
    impl Fieldset {
        /// Get the value of the `disabled` attribute
        pub fn disabled(&self) -> bool {
            self.sys.disabled
        }
        /// Set the value of the `disabled` attribute
        pub fn set_disabled(&mut self, value: bool) {
            self.sys.disabled = value;
        }
        /// Get the value of the `form` attribute
        pub fn form(&self) -> std::option::Option<&str> {
            self.sys.form.as_deref()
        }
        /// Set the value of the `form` attribute
        pub fn set_form(&mut self, value: std::option::Option<String>) {
            self.sys.form = value;
        }
        /// Get the value of the `name` attribute
        pub fn name(&self) -> std::option::Option<&str> {
            self.sys.name.as_deref()
        }
        /// Set the value of the `name` attribute
        pub fn set_name(&mut self, value: std::option::Option<String>) {
            self.sys.name = value;
        }
    }
    impl crate::HtmlElement for Fieldset {}
    impl crate::FlowContent for Fieldset {}
    impl crate::PalpableContent for Fieldset {}
    impl std::convert::Into<html_sys::forms::Fieldset> for Fieldset {
        fn into(self) -> html_sys::forms::Fieldset {
            self.sys
        }
    }
    impl From<html_sys::forms::Fieldset> for Fieldset {
        fn from(sys: html_sys::forms::Fieldset) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Fieldset` element
    pub enum FieldsetChild {
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
