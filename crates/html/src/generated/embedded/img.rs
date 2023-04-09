pub mod element {
    /// The HTML `<img>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
    #[doc(alias = "img")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Image {
        sys: html_sys::embedded::Image,
        children: Vec<super::child::ImageChild>,
    }
    impl Image {
        /// Get the value of the `alt` attribute
        pub fn alt(&self) -> std::option::Option<&str> {
            self.sys.alt.as_deref()
        }
        /// Set the value of the `alt` attribute
        pub fn set_alt(&mut self, value: std::option::Option<String>) {
            self.sys.alt = value;
        }
        /// Get the value of the `src` attribute
        pub fn src(&self) -> std::option::Option<&str> {
            self.sys.src.as_deref()
        }
        /// Set the value of the `src` attribute
        pub fn set_src(&mut self, value: std::option::Option<String>) {
            self.sys.src = value;
        }
        /// Get the value of the `srcset` attribute
        pub fn srcset(&self) -> std::option::Option<&str> {
            self.sys.srcset.as_deref()
        }
        /// Set the value of the `srcset` attribute
        pub fn set_srcset(&mut self, value: std::option::Option<String>) {
            self.sys.srcset = value;
        }
        /// Get the value of the `sizes` attribute
        pub fn sizes(&self) -> std::option::Option<&str> {
            self.sys.sizes.as_deref()
        }
        /// Set the value of the `sizes` attribute
        pub fn set_sizes(&mut self, value: std::option::Option<String>) {
            self.sys.sizes = value;
        }
        /// Get the value of the `crossorigin` attribute
        pub fn crossorigin(&self) -> std::option::Option<&str> {
            self.sys.crossorigin.as_deref()
        }
        /// Set the value of the `crossorigin` attribute
        pub fn set_crossorigin(&mut self, value: std::option::Option<String>) {
            self.sys.crossorigin = value;
        }
        /// Get the value of the `usemap` attribute
        pub fn usemap(&self) -> std::option::Option<&str> {
            self.sys.usemap.as_deref()
        }
        /// Set the value of the `usemap` attribute
        pub fn set_usemap(&mut self, value: std::option::Option<String>) {
            self.sys.usemap = value;
        }
        /// Get the value of the `ismap` attribute
        pub fn ismap(&self) -> std::option::Option<&str> {
            self.sys.ismap.as_deref()
        }
        /// Set the value of the `ismap` attribute
        pub fn set_ismap(&mut self, value: std::option::Option<String>) {
            self.sys.ismap = value;
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
        /// Get the value of the `referrerpolicy` attribute
        pub fn referrerpolicy(&self) -> std::option::Option<&str> {
            self.sys.referrerpolicy.as_deref()
        }
        /// Set the value of the `referrerpolicy` attribute
        pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
            self.sys.referrerpolicy = value;
        }
        /// Get the value of the `decoding` attribute
        pub fn decoding(&self) -> std::option::Option<&str> {
            self.sys.decoding.as_deref()
        }
        /// Set the value of the `decoding` attribute
        pub fn set_decoding(&mut self, value: std::option::Option<String>) {
            self.sys.decoding = value;
        }
        /// Get the value of the `loading` attribute
        pub fn loading(&self) -> std::option::Option<&str> {
            self.sys.loading.as_deref()
        }
        /// Set the value of the `loading` attribute
        pub fn set_loading(&mut self, value: std::option::Option<String>) {
            self.sys.loading = value;
        }
        /// Get the value of the `fetchpriority` attribute
        pub fn fetchpriority(&self) -> std::option::Option<&str> {
            self.sys.fetchpriority.as_deref()
        }
        /// Set the value of the `fetchpriority` attribute
        pub fn set_fetchpriority(&mut self, value: std::option::Option<String>) {
            self.sys.fetchpriority = value;
        }
    }
    impl Image {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::ImageChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::ImageChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Image {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Image {}
    impl crate::FlowContent for Image {}
    impl crate::PhrasingContent for Image {}
    impl crate::EmbeddedContent for Image {}
    impl crate::PalpableContent for Image {}
    impl std::convert::Into<html_sys::embedded::Image> for Image {
        fn into(self) -> html_sys::embedded::Image {
            self.sys
        }
    }
    impl From<html_sys::embedded::Image> for Image {
        fn from(sys: html_sys::embedded::Image) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Image` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub enum ImageChild {
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
    impl std::convert::From<crate::generated::all::Abbreviation> for ImageChild {
        fn from(value: crate::generated::all::Abbreviation) -> Self {
            Self::Abbreviation(value)
        }
    }
    impl std::convert::From<crate::generated::all::Address> for ImageChild {
        fn from(value: crate::generated::all::Address) -> Self {
            Self::Address(value)
        }
    }
    impl std::convert::From<crate::generated::all::Anchor> for ImageChild {
        fn from(value: crate::generated::all::Anchor) -> Self {
            Self::Anchor(value)
        }
    }
    impl std::convert::From<crate::generated::all::Audio> for ImageChild {
        fn from(value: crate::generated::all::Audio) -> Self {
            Self::Audio(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalIsolate> for ImageChild {
        fn from(value: crate::generated::all::BidirectionalIsolate) -> Self {
            Self::BidirectionalIsolate(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalTextOverride>
    for ImageChild {
        fn from(value: crate::generated::all::BidirectionalTextOverride) -> Self {
            Self::BidirectionalTextOverride(value)
        }
    }
    impl std::convert::From<crate::generated::all::BlockQuote> for ImageChild {
        fn from(value: crate::generated::all::BlockQuote) -> Self {
            Self::BlockQuote(value)
        }
    }
    impl std::convert::From<crate::generated::all::Bold> for ImageChild {
        fn from(value: crate::generated::all::Bold) -> Self {
            Self::Bold(value)
        }
    }
    impl std::convert::From<crate::generated::all::Button> for ImageChild {
        fn from(value: crate::generated::all::Button) -> Self {
            Self::Button(value)
        }
    }
    impl std::convert::From<crate::generated::all::Canvas> for ImageChild {
        fn from(value: crate::generated::all::Canvas) -> Self {
            Self::Canvas(value)
        }
    }
    impl std::convert::From<crate::generated::all::Cite> for ImageChild {
        fn from(value: crate::generated::all::Cite) -> Self {
            Self::Cite(value)
        }
    }
    impl std::convert::From<crate::generated::all::Code> for ImageChild {
        fn from(value: crate::generated::all::Code) -> Self {
            Self::Code(value)
        }
    }
    impl std::convert::From<crate::generated::all::Data> for ImageChild {
        fn from(value: crate::generated::all::Data) -> Self {
            Self::Data(value)
        }
    }
    impl std::convert::From<crate::generated::all::DataList> for ImageChild {
        fn from(value: crate::generated::all::DataList) -> Self {
            Self::DataList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Definition> for ImageChild {
        fn from(value: crate::generated::all::Definition) -> Self {
            Self::Definition(value)
        }
    }
    impl std::convert::From<crate::generated::all::DeletedText> for ImageChild {
        fn from(value: crate::generated::all::DeletedText) -> Self {
            Self::DeletedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::DescriptionList> for ImageChild {
        fn from(value: crate::generated::all::DescriptionList) -> Self {
            Self::DescriptionList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Details> for ImageChild {
        fn from(value: crate::generated::all::Details) -> Self {
            Self::Details(value)
        }
    }
    impl std::convert::From<crate::generated::all::Dialog> for ImageChild {
        fn from(value: crate::generated::all::Dialog) -> Self {
            Self::Dialog(value)
        }
    }
    impl std::convert::From<crate::generated::all::Division> for ImageChild {
        fn from(value: crate::generated::all::Division) -> Self {
            Self::Division(value)
        }
    }
    impl std::convert::From<crate::generated::all::Embed> for ImageChild {
        fn from(value: crate::generated::all::Embed) -> Self {
            Self::Embed(value)
        }
    }
    impl std::convert::From<crate::generated::all::Emphasis> for ImageChild {
        fn from(value: crate::generated::all::Emphasis) -> Self {
            Self::Emphasis(value)
        }
    }
    impl std::convert::From<crate::generated::all::Fieldset> for ImageChild {
        fn from(value: crate::generated::all::Fieldset) -> Self {
            Self::Fieldset(value)
        }
    }
    impl std::convert::From<crate::generated::all::Figure> for ImageChild {
        fn from(value: crate::generated::all::Figure) -> Self {
            Self::Figure(value)
        }
    }
    impl std::convert::From<crate::generated::all::Footer> for ImageChild {
        fn from(value: crate::generated::all::Footer) -> Self {
            Self::Footer(value)
        }
    }
    impl std::convert::From<crate::generated::all::Form> for ImageChild {
        fn from(value: crate::generated::all::Form) -> Self {
            Self::Form(value)
        }
    }
    impl std::convert::From<crate::generated::all::Header> for ImageChild {
        fn from(value: crate::generated::all::Header) -> Self {
            Self::Header(value)
        }
    }
    impl std::convert::From<crate::generated::all::Iframe> for ImageChild {
        fn from(value: crate::generated::all::Iframe) -> Self {
            Self::Iframe(value)
        }
    }
    impl std::convert::From<crate::generated::all::Image> for ImageChild {
        fn from(value: crate::generated::all::Image) -> Self {
            Self::Image(value)
        }
    }
    impl std::convert::From<crate::generated::all::ImageMap> for ImageChild {
        fn from(value: crate::generated::all::ImageMap) -> Self {
            Self::ImageMap(value)
        }
    }
    impl std::convert::From<crate::generated::all::Input> for ImageChild {
        fn from(value: crate::generated::all::Input) -> Self {
            Self::Input(value)
        }
    }
    impl std::convert::From<crate::generated::all::InsertedText> for ImageChild {
        fn from(value: crate::generated::all::InsertedText) -> Self {
            Self::InsertedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Italic> for ImageChild {
        fn from(value: crate::generated::all::Italic) -> Self {
            Self::Italic(value)
        }
    }
    impl std::convert::From<crate::generated::all::KeyboardInput> for ImageChild {
        fn from(value: crate::generated::all::KeyboardInput) -> Self {
            Self::KeyboardInput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Label> for ImageChild {
        fn from(value: crate::generated::all::Label) -> Self {
            Self::Label(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreak> for ImageChild {
        fn from(value: crate::generated::all::LineBreak) -> Self {
            Self::LineBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreakOpportunity> for ImageChild {
        fn from(value: crate::generated::all::LineBreakOpportunity) -> Self {
            Self::LineBreakOpportunity(value)
        }
    }
    impl std::convert::From<crate::generated::all::MarkText> for ImageChild {
        fn from(value: crate::generated::all::MarkText) -> Self {
            Self::MarkText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Menu> for ImageChild {
        fn from(value: crate::generated::all::Menu) -> Self {
            Self::Menu(value)
        }
    }
    impl std::convert::From<crate::generated::all::Meter> for ImageChild {
        fn from(value: crate::generated::all::Meter) -> Self {
            Self::Meter(value)
        }
    }
    impl std::convert::From<crate::generated::all::Object> for ImageChild {
        fn from(value: crate::generated::all::Object) -> Self {
            Self::Object(value)
        }
    }
    impl std::convert::From<crate::generated::all::OrderedList> for ImageChild {
        fn from(value: crate::generated::all::OrderedList) -> Self {
            Self::OrderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Output> for ImageChild {
        fn from(value: crate::generated::all::Output) -> Self {
            Self::Output(value)
        }
    }
    impl std::convert::From<crate::generated::all::Paragraph> for ImageChild {
        fn from(value: crate::generated::all::Paragraph) -> Self {
            Self::Paragraph(value)
        }
    }
    impl std::convert::From<crate::generated::all::Picture> for ImageChild {
        fn from(value: crate::generated::all::Picture) -> Self {
            Self::Picture(value)
        }
    }
    impl std::convert::From<crate::generated::all::PreformattedText> for ImageChild {
        fn from(value: crate::generated::all::PreformattedText) -> Self {
            Self::PreformattedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Progress> for ImageChild {
        fn from(value: crate::generated::all::Progress) -> Self {
            Self::Progress(value)
        }
    }
    impl std::convert::From<crate::generated::all::Quotation> for ImageChild {
        fn from(value: crate::generated::all::Quotation) -> Self {
            Self::Quotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::RubyAnnotation> for ImageChild {
        fn from(value: crate::generated::all::RubyAnnotation) -> Self {
            Self::RubyAnnotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::SampleOutput> for ImageChild {
        fn from(value: crate::generated::all::SampleOutput) -> Self {
            Self::SampleOutput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for ImageChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Select> for ImageChild {
        fn from(value: crate::generated::all::Select) -> Self {
            Self::Select(value)
        }
    }
    impl std::convert::From<crate::generated::all::SideComment> for ImageChild {
        fn from(value: crate::generated::all::SideComment) -> Self {
            Self::SideComment(value)
        }
    }
    impl std::convert::From<crate::generated::all::Slot> for ImageChild {
        fn from(value: crate::generated::all::Slot) -> Self {
            Self::Slot(value)
        }
    }
    impl std::convert::From<crate::generated::all::Span> for ImageChild {
        fn from(value: crate::generated::all::Span) -> Self {
            Self::Span(value)
        }
    }
    impl std::convert::From<crate::generated::all::StrikeThrough> for ImageChild {
        fn from(value: crate::generated::all::StrikeThrough) -> Self {
            Self::StrikeThrough(value)
        }
    }
    impl std::convert::From<crate::generated::all::Strong> for ImageChild {
        fn from(value: crate::generated::all::Strong) -> Self {
            Self::Strong(value)
        }
    }
    impl std::convert::From<crate::generated::all::SubScript> for ImageChild {
        fn from(value: crate::generated::all::SubScript) -> Self {
            Self::SubScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::SuperScript> for ImageChild {
        fn from(value: crate::generated::all::SuperScript) -> Self {
            Self::SuperScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::Table> for ImageChild {
        fn from(value: crate::generated::all::Table) -> Self {
            Self::Table(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for ImageChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::convert::From<crate::generated::all::TextArea> for ImageChild {
        fn from(value: crate::generated::all::TextArea) -> Self {
            Self::TextArea(value)
        }
    }
    impl std::convert::From<crate::generated::all::ThematicBreak> for ImageChild {
        fn from(value: crate::generated::all::ThematicBreak) -> Self {
            Self::ThematicBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::Time> for ImageChild {
        fn from(value: crate::generated::all::Time) -> Self {
            Self::Time(value)
        }
    }
    impl std::convert::From<crate::generated::all::Underline> for ImageChild {
        fn from(value: crate::generated::all::Underline) -> Self {
            Self::Underline(value)
        }
    }
    impl std::convert::From<crate::generated::all::UnorderedList> for ImageChild {
        fn from(value: crate::generated::all::UnorderedList) -> Self {
            Self::UnorderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Variable> for ImageChild {
        fn from(value: crate::generated::all::Variable) -> Self {
            Self::Variable(value)
        }
    }
    impl std::convert::From<crate::generated::all::Video> for ImageChild {
        fn from(value: crate::generated::all::Video) -> Self {
            Self::Video(value)
        }
    }
    impl std::fmt::Display for ImageChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Abbreviation(el) => write!(f, "{el}"),
                Self::Address(el) => write!(f, "{el}"),
                Self::Anchor(el) => write!(f, "{el}"),
                Self::Audio(el) => write!(f, "{el}"),
                Self::BidirectionalIsolate(el) => write!(f, "{el}"),
                Self::BidirectionalTextOverride(el) => write!(f, "{el}"),
                Self::BlockQuote(el) => write!(f, "{el}"),
                Self::Bold(el) => write!(f, "{el}"),
                Self::Button(el) => write!(f, "{el}"),
                Self::Canvas(el) => write!(f, "{el}"),
                Self::Cite(el) => write!(f, "{el}"),
                Self::Code(el) => write!(f, "{el}"),
                Self::Data(el) => write!(f, "{el}"),
                Self::DataList(el) => write!(f, "{el}"),
                Self::Definition(el) => write!(f, "{el}"),
                Self::DeletedText(el) => write!(f, "{el}"),
                Self::DescriptionList(el) => write!(f, "{el}"),
                Self::Details(el) => write!(f, "{el}"),
                Self::Dialog(el) => write!(f, "{el}"),
                Self::Division(el) => write!(f, "{el}"),
                Self::Embed(el) => write!(f, "{el}"),
                Self::Emphasis(el) => write!(f, "{el}"),
                Self::Fieldset(el) => write!(f, "{el}"),
                Self::Figure(el) => write!(f, "{el}"),
                Self::Footer(el) => write!(f, "{el}"),
                Self::Form(el) => write!(f, "{el}"),
                Self::Header(el) => write!(f, "{el}"),
                Self::Iframe(el) => write!(f, "{el}"),
                Self::Image(el) => write!(f, "{el}"),
                Self::ImageMap(el) => write!(f, "{el}"),
                Self::Input(el) => write!(f, "{el}"),
                Self::InsertedText(el) => write!(f, "{el}"),
                Self::Italic(el) => write!(f, "{el}"),
                Self::KeyboardInput(el) => write!(f, "{el}"),
                Self::Label(el) => write!(f, "{el}"),
                Self::LineBreak(el) => write!(f, "{el}"),
                Self::LineBreakOpportunity(el) => write!(f, "{el}"),
                Self::MarkText(el) => write!(f, "{el}"),
                Self::Menu(el) => write!(f, "{el}"),
                Self::Meter(el) => write!(f, "{el}"),
                Self::Object(el) => write!(f, "{el}"),
                Self::OrderedList(el) => write!(f, "{el}"),
                Self::Output(el) => write!(f, "{el}"),
                Self::Paragraph(el) => write!(f, "{el}"),
                Self::Picture(el) => write!(f, "{el}"),
                Self::PreformattedText(el) => write!(f, "{el}"),
                Self::Progress(el) => write!(f, "{el}"),
                Self::Quotation(el) => write!(f, "{el}"),
                Self::RubyAnnotation(el) => write!(f, "{el}"),
                Self::SampleOutput(el) => write!(f, "{el}"),
                Self::Script(el) => write!(f, "{el}"),
                Self::Select(el) => write!(f, "{el}"),
                Self::SideComment(el) => write!(f, "{el}"),
                Self::Slot(el) => write!(f, "{el}"),
                Self::Span(el) => write!(f, "{el}"),
                Self::StrikeThrough(el) => write!(f, "{el}"),
                Self::Strong(el) => write!(f, "{el}"),
                Self::SubScript(el) => write!(f, "{el}"),
                Self::SuperScript(el) => write!(f, "{el}"),
                Self::Table(el) => write!(f, "{el}"),
                Self::Template(el) => write!(f, "{el}"),
                Self::TextArea(el) => write!(f, "{el}"),
                Self::ThematicBreak(el) => write!(f, "{el}"),
                Self::Time(el) => write!(f, "{el}"),
                Self::Underline(el) => write!(f, "{el}"),
                Self::UnorderedList(el) => write!(f, "{el}"),
                Self::Variable(el) => write!(f, "{el}"),
                Self::Video(el) => write!(f, "{el}"),
            }
        }
    }
}
