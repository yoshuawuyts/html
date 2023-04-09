pub mod element {
    /// The HTML `<button>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    #[doc(alias = "button")]
    #[non_exhaustive]
    pub struct Button {
        sys: html_sys::forms::Button,
        _children: Vec<super::child::ButtonChild>,
    }
    impl Button {
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
        /// Get the value of the `formaction` attribute
        pub fn form_action(&self) -> std::option::Option<&str> {
            self.sys.form_action.as_deref()
        }
        /// Set the value of the `formaction` attribute
        pub fn set_form_action(&mut self, value: std::option::Option<String>) {
            self.sys.form_action = value;
        }
        /// Get the value of the `formenctype` attribute
        pub fn form_enctype(&self) -> std::option::Option<&str> {
            self.sys.form_enctype.as_deref()
        }
        /// Set the value of the `formenctype` attribute
        pub fn set_form_enctype(&mut self, value: std::option::Option<String>) {
            self.sys.form_enctype = value;
        }
        /// Get the value of the `formmethod` attribute
        pub fn form_method(&self) -> std::option::Option<&str> {
            self.sys.form_method.as_deref()
        }
        /// Set the value of the `formmethod` attribute
        pub fn set_form_method(&mut self, value: std::option::Option<String>) {
            self.sys.form_method = value;
        }
        /// Get the value of the `formnovalidate` attribute
        pub fn form_no_validate(&self) -> bool {
            self.sys.form_no_validate
        }
        /// Set the value of the `formnovalidate` attribute
        pub fn set_form_no_validate(&mut self, value: bool) {
            self.sys.form_no_validate = value;
        }
        /// Get the value of the `formtarget` attribute
        pub fn form_target(&self) -> std::option::Option<&str> {
            self.sys.form_target.as_deref()
        }
        /// Set the value of the `formtarget` attribute
        pub fn set_form_target(&mut self, value: std::option::Option<String>) {
            self.sys.form_target = value;
        }
        /// Get the value of the `name` attribute
        pub fn name(&self) -> std::option::Option<&str> {
            self.sys.name.as_deref()
        }
        /// Set the value of the `name` attribute
        pub fn set_name(&mut self, value: std::option::Option<String>) {
            self.sys.name = value;
        }
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(&mut self, value: std::option::Option<String>) {
            self.sys.type_ = value;
        }
        /// Get the value of the `value` attribute
        pub fn value(&self) -> std::option::Option<&str> {
            self.sys.value.as_deref()
        }
        /// Set the value of the `value` attribute
        pub fn set_value(&mut self, value: std::option::Option<String>) {
            self.sys.value = value;
        }
    }
    impl crate::HtmlElement for Button {}
    impl crate::FlowContent for Button {}
    impl crate::PhrasingContent for Button {}
    impl crate::InteractiveContent for Button {}
    impl crate::PalpableContent for Button {}
    impl std::convert::Into<html_sys::forms::Button> for Button {
        fn into(self) -> html_sys::forms::Button {
            self.sys
        }
    }
    impl From<html_sys::forms::Button> for Button {
        fn from(sys: html_sys::forms::Button) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Button` element
    pub enum ButtonChild {
        /// The Abbreviation element
        Abbreviation(crate::generated::all::Abbreviation),
        /// The Address element
        Address(crate::generated::all::Address),
        /// The Anchor element
        Anchor(crate::generated::all::Anchor),
        /// The BidirectionalIsolate element
        BidirectionalIsolate(crate::generated::all::BidirectionalIsolate),
        /// The BidirectionalTextOverride element
        BidirectionalTextOverride(crate::generated::all::BidirectionalTextOverride),
        /// The BlockQuote element
        BlockQuote(crate::generated::all::BlockQuote),
        /// The Bold element
        Bold(crate::generated::all::Bold),
        /// The Button element
        Button(crate::generated::all::Button),
        /// The Cite element
        Cite(crate::generated::all::Cite),
        /// The Code element
        Code(crate::generated::all::Code),
        /// The Data element
        Data(crate::generated::all::Data),
        /// The DataList element
        DataList(crate::generated::all::DataList),
        /// The Definition element
        Definition(crate::generated::all::Definition),
        /// The DeletedText element
        DeletedText(crate::generated::all::DeletedText),
        /// The DescriptionList element
        DescriptionList(crate::generated::all::DescriptionList),
        /// The Details element
        Details(crate::generated::all::Details),
        /// The Dialog element
        Dialog(crate::generated::all::Dialog),
        /// The Division element
        Division(crate::generated::all::Division),
        /// The Emphasis element
        Emphasis(crate::generated::all::Emphasis),
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
        /// The ImageMap element
        ImageMap(crate::generated::all::ImageMap),
        /// The Input element
        Input(crate::generated::all::Input),
        /// The InsertedText element
        InsertedText(crate::generated::all::InsertedText),
        /// The Italic element
        Italic(crate::generated::all::Italic),
        /// The KeyboardInput element
        KeyboardInput(crate::generated::all::KeyboardInput),
        /// The Label element
        Label(crate::generated::all::Label),
        /// The LineBreak element
        LineBreak(crate::generated::all::LineBreak),
        /// The LineBreakOpportunity element
        LineBreakOpportunity(crate::generated::all::LineBreakOpportunity),
        /// The MarkText element
        MarkText(crate::generated::all::MarkText),
        /// The Menu element
        Menu(crate::generated::all::Menu),
        /// The Meter element
        Meter(crate::generated::all::Meter),
        /// The OrderedList element
        OrderedList(crate::generated::all::OrderedList),
        /// The Output element
        Output(crate::generated::all::Output),
        /// The Paragraph element
        Paragraph(crate::generated::all::Paragraph),
        /// The PreformattedText element
        PreformattedText(crate::generated::all::PreformattedText),
        /// The Progress element
        Progress(crate::generated::all::Progress),
        /// The Quotation element
        Quotation(crate::generated::all::Quotation),
        /// The RubyAnnotation element
        RubyAnnotation(crate::generated::all::RubyAnnotation),
        /// The SampleOutput element
        SampleOutput(crate::generated::all::SampleOutput),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Select element
        Select(crate::generated::all::Select),
        /// The SideComment element
        SideComment(crate::generated::all::SideComment),
        /// The Slot element
        Slot(crate::generated::all::Slot),
        /// The Span element
        Span(crate::generated::all::Span),
        /// The StrikeThrough element
        StrikeThrough(crate::generated::all::StrikeThrough),
        /// The Strong element
        Strong(crate::generated::all::Strong),
        /// The SubScript element
        SubScript(crate::generated::all::SubScript),
        /// The SuperScript element
        SuperScript(crate::generated::all::SuperScript),
        /// The Table element
        Table(crate::generated::all::Table),
        /// The Template element
        Template(crate::generated::all::Template),
        /// The TextArea element
        TextArea(crate::generated::all::TextArea),
        /// The ThematicBreak element
        ThematicBreak(crate::generated::all::ThematicBreak),
        /// The Time element
        Time(crate::generated::all::Time),
        /// The Underline element
        Underline(crate::generated::all::Underline),
        /// The UnorderedList element
        UnorderedList(crate::generated::all::UnorderedList),
        /// The Variable element
        Variable(crate::generated::all::Variable),
    }
}
