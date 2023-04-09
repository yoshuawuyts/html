pub mod element {
    /// The HTML `<object>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object)
    #[doc(alias = "object")]
    #[non_exhaustive]
    pub struct Object {
        sys: html_sys::embedded::Object,
        _children: Vec<super::child::ObjectChild>,
    }
    impl Object {
        /// Get the value of the `data` attribute
        pub fn data(&self) -> std::option::Option<&str> {
            self.sys.data.as_deref()
        }
        /// Set the value of the `data` attribute
        pub fn set_data(&mut self, value: std::option::Option<String>) {
            self.sys.data = value;
        }
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(&mut self, value: std::option::Option<String>) {
            self.sys.type_ = value;
        }
        /// Get the value of the `name` attribute
        pub fn name(&self) -> std::option::Option<&str> {
            self.sys.name.as_deref()
        }
        /// Set the value of the `name` attribute
        pub fn set_name(&mut self, value: std::option::Option<String>) {
            self.sys.name = value;
        }
        /// Get the value of the `form` attribute
        pub fn form(&self) -> std::option::Option<&str> {
            self.sys.form.as_deref()
        }
        /// Set the value of the `form` attribute
        pub fn set_form(&mut self, value: std::option::Option<String>) {
            self.sys.form = value;
        }
        /// Get the value of the `width` attribute
        pub fn width(&self) -> std::option::Option<&str> {
            self.sys.width.as_deref()
        }
        /// Set the value of the `width` attribute
        pub fn set_width(&mut self, value: std::option::Option<String>) {
            self.sys.width = value;
        }
        /// Get the value of the `height` attribute
        pub fn height(&self) -> std::option::Option<&str> {
            self.sys.height.as_deref()
        }
        /// Set the value of the `height` attribute
        pub fn set_height(&mut self, value: std::option::Option<String>) {
            self.sys.height = value;
        }
    }
    impl crate::HtmlElement for Object {}
    impl crate::FlowContent for Object {}
    impl crate::PhrasingContent for Object {}
    impl crate::EmbeddedContent for Object {}
    impl crate::PalpableContent for Object {}
    impl std::convert::Into<html_sys::embedded::Object> for Object {
        fn into(self) -> html_sys::embedded::Object {
            self.sys
        }
    }
    impl From<html_sys::embedded::Object> for Object {
        fn from(sys: html_sys::embedded::Object) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Object` element
    pub enum ObjectChild {
        /// The Abbreviation element
        Abbreviation(crate::generated::all::Abbreviation),
        /// The Address element
        Address(crate::generated::all::Address),
        /// The Anchor element
        Anchor(crate::generated::all::Anchor),
        /// The Audio element
        Audio(crate::generated::all::Audio),
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
        /// The Canvas element
        Canvas(crate::generated::all::Canvas),
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
        /// The Embed element
        Embed(crate::generated::all::Embed),
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
        /// The Iframe element
        Iframe(crate::generated::all::Iframe),
        /// The Image element
        Image(crate::generated::all::Image),
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
        /// The Object element
        Object(crate::generated::all::Object),
        /// The OrderedList element
        OrderedList(crate::generated::all::OrderedList),
        /// The Output element
        Output(crate::generated::all::Output),
        /// The Paragraph element
        Paragraph(crate::generated::all::Paragraph),
        /// The Picture element
        Picture(crate::generated::all::Picture),
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
        /// The Video element
        Video(crate::generated::all::Video),
    }
}
