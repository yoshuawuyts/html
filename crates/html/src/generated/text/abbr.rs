pub mod element {
    /// The HTML `<abbr>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
    #[doc(alias = "abbr")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Abbreviation {
        sys: html_sys::text::Abbreviation,
        children: Vec<super::child::AbbreviationChild>,
    }
    impl Abbreviation {
        /// Get the value of the `accesskey` attribute
        pub fn access_key(&self) -> std::option::Option<&str> {
            self.sys.access_key.as_deref()
        }
        /// Set the value of the `accesskey` attribute
        pub fn set_access_key(&mut self, value: std::option::Option<String>) {
            self.sys.access_key = value;
        }
        /// Get the value of the `autocapitalize` attribute
        pub fn auto_capitalize(&self) -> std::option::Option<&str> {
            self.sys.auto_capitalize.as_deref()
        }
        /// Set the value of the `autocapitalize` attribute
        pub fn set_auto_capitalize(&mut self, value: std::option::Option<String>) {
            self.sys.auto_capitalize = value;
        }
        /// Get the value of the `autofocus` attribute
        pub fn autofocus(&self) -> bool {
            self.sys.autofocus
        }
        /// Set the value of the `autofocus` attribute
        pub fn set_autofocus(&mut self, value: bool) {
            self.sys.autofocus = value;
        }
        /// Get the value of the `class` attribute
        pub fn class_(&self) -> std::option::Option<&str> {
            self.sys.class_.as_deref()
        }
        /// Set the value of the `class` attribute
        pub fn set_class_(&mut self, value: std::option::Option<String>) {
            self.sys.class_ = value;
        }
        /// Get the value of the `contenteditable` attribute
        pub fn content_editable(&self) -> std::option::Option<&str> {
            self.sys.content_editable.as_deref()
        }
        /// Set the value of the `contenteditable` attribute
        pub fn set_content_editable(&mut self, value: std::option::Option<String>) {
            self.sys.content_editable = value;
        }
        /// Get the value of the `dir` attribute
        pub fn direction(&self) -> std::option::Option<&str> {
            self.sys.direction.as_deref()
        }
        /// Set the value of the `dir` attribute
        pub fn set_direction(&mut self, value: std::option::Option<String>) {
            self.sys.direction = value;
        }
        /// Get the value of the `draggable` attribute
        pub fn draggable(&self) -> bool {
            self.sys.draggable
        }
        /// Set the value of the `draggable` attribute
        pub fn set_draggable(&mut self, value: bool) {
            self.sys.draggable = value;
        }
        /// Get the value of the `enterkeyhint` attribute
        pub fn enter_key_hint(&self) -> std::option::Option<&str> {
            self.sys.enter_key_hint.as_deref()
        }
        /// Set the value of the `enterkeyhint` attribute
        pub fn set_enter_key_hint(&mut self, value: std::option::Option<String>) {
            self.sys.enter_key_hint = value;
        }
        /// Get the value of the `exportparts` attribute
        pub fn export_parts(&self) -> std::option::Option<&str> {
            self.sys.export_parts.as_deref()
        }
        /// Set the value of the `exportparts` attribute
        pub fn set_export_parts(&mut self, value: std::option::Option<String>) {
            self.sys.export_parts = value;
        }
        /// Get the value of the `hidden` attribute
        pub fn hidden(&self) -> std::option::Option<&str> {
            self.sys.hidden.as_deref()
        }
        /// Set the value of the `hidden` attribute
        pub fn set_hidden(&mut self, value: std::option::Option<String>) {
            self.sys.hidden = value;
        }
        /// Get the value of the `id` attribute
        pub fn id(&self) -> std::option::Option<&str> {
            self.sys.id.as_deref()
        }
        /// Set the value of the `id` attribute
        pub fn set_id(&mut self, value: std::option::Option<String>) {
            self.sys.id = value;
        }
        /// Get the value of the `inert` attribute
        pub fn inert(&self) -> bool {
            self.sys.inert
        }
        /// Set the value of the `inert` attribute
        pub fn set_inert(&mut self, value: bool) {
            self.sys.inert = value;
        }
        /// Get the value of the `inputmode` attribute
        pub fn input_mode(&self) -> std::option::Option<&str> {
            self.sys.input_mode.as_deref()
        }
        /// Set the value of the `inputmode` attribute
        pub fn set_input_mode(&mut self, value: std::option::Option<String>) {
            self.sys.input_mode = value;
        }
        /// Get the value of the `is` attribute
        pub fn is_(&self) -> std::option::Option<&str> {
            self.sys.is_.as_deref()
        }
        /// Set the value of the `is` attribute
        pub fn set_is_(&mut self, value: std::option::Option<String>) {
            self.sys.is_ = value;
        }
        /// Get the value of the `itemid` attribute
        pub fn item_id(&self) -> std::option::Option<&str> {
            self.sys.item_id.as_deref()
        }
        /// Set the value of the `itemid` attribute
        pub fn set_item_id(&mut self, value: std::option::Option<String>) {
            self.sys.item_id = value;
        }
        /// Get the value of the `itemprop` attribute
        pub fn item_prop(&self) -> std::option::Option<&str> {
            self.sys.item_prop.as_deref()
        }
        /// Set the value of the `itemprop` attribute
        pub fn set_item_prop(&mut self, value: std::option::Option<String>) {
            self.sys.item_prop = value;
        }
        /// Get the value of the `itemref` attribute
        pub fn item_ref(&self) -> std::option::Option<&str> {
            self.sys.item_ref.as_deref()
        }
        /// Set the value of the `itemref` attribute
        pub fn set_item_ref(&mut self, value: std::option::Option<String>) {
            self.sys.item_ref = value;
        }
        /// Get the value of the `itemscope` attribute
        pub fn item_scope(&self) -> std::option::Option<&str> {
            self.sys.item_scope.as_deref()
        }
        /// Set the value of the `itemscope` attribute
        pub fn set_item_scope(&mut self, value: std::option::Option<String>) {
            self.sys.item_scope = value;
        }
        /// Get the value of the `itemtype` attribute
        pub fn item_type(&self) -> std::option::Option<&str> {
            self.sys.item_type.as_deref()
        }
        /// Set the value of the `itemtype` attribute
        pub fn set_item_type(&mut self, value: std::option::Option<String>) {
            self.sys.item_type = value;
        }
        /// Get the value of the `lang` attribute
        pub fn lang(&self) -> std::option::Option<&str> {
            self.sys.lang.as_deref()
        }
        /// Set the value of the `lang` attribute
        pub fn set_lang(&mut self, value: std::option::Option<String>) {
            self.sys.lang = value;
        }
        /// Get the value of the `nonce` attribute
        pub fn nonce(&self) -> std::option::Option<&str> {
            self.sys.nonce.as_deref()
        }
        /// Set the value of the `nonce` attribute
        pub fn set_nonce(&mut self, value: std::option::Option<String>) {
            self.sys.nonce = value;
        }
        /// Get the value of the `part` attribute
        pub fn part(&self) -> std::option::Option<&str> {
            self.sys.part.as_deref()
        }
        /// Set the value of the `part` attribute
        pub fn set_part(&mut self, value: std::option::Option<String>) {
            self.sys.part = value;
        }
        /// Get the value of the `slot` attribute
        pub fn slot(&self) -> std::option::Option<&str> {
            self.sys.slot.as_deref()
        }
        /// Set the value of the `slot` attribute
        pub fn set_slot(&mut self, value: std::option::Option<String>) {
            self.sys.slot = value;
        }
        /// Get the value of the `spellcheck` attribute
        pub fn spellcheck(&self) -> std::option::Option<&str> {
            self.sys.spellcheck.as_deref()
        }
        /// Set the value of the `spellcheck` attribute
        pub fn set_spellcheck(&mut self, value: std::option::Option<String>) {
            self.sys.spellcheck = value;
        }
        /// Get the value of the `style` attribute
        pub fn style(&self) -> std::option::Option<&str> {
            self.sys.style.as_deref()
        }
        /// Set the value of the `style` attribute
        pub fn set_style(&mut self, value: std::option::Option<String>) {
            self.sys.style = value;
        }
        /// Get the value of the `tabindex` attribute
        pub fn tab_index(&self) -> std::option::Option<i64> {
            self.sys.tab_index
        }
        /// Set the value of the `tabindex` attribute
        pub fn set_tab_index(&mut self, value: std::option::Option<i64>) {
            self.sys.tab_index = value;
        }
        /// Get the value of the `title` attribute
        pub fn title(&self) -> std::option::Option<&str> {
            self.sys.title.as_deref()
        }
        /// Set the value of the `title` attribute
        pub fn set_title(&mut self, value: std::option::Option<String>) {
            self.sys.title = value;
        }
        /// Get the value of the `translate` attribute
        pub fn translate(&self) -> bool {
            self.sys.translate
        }
        /// Set the value of the `translate` attribute
        pub fn set_translate(&mut self, value: bool) {
            self.sys.translate = value;
        }
    }
    impl Abbreviation {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::AbbreviationChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::AbbreviationChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Abbreviation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Abbreviation {}
    impl crate::FlowContent for Abbreviation {}
    impl crate::PhrasingContent for Abbreviation {}
    impl crate::PalpableContent for Abbreviation {}
    impl std::convert::Into<html_sys::text::Abbreviation> for Abbreviation {
        fn into(self) -> html_sys::text::Abbreviation {
            self.sys
        }
    }
    impl From<html_sys::text::Abbreviation> for Abbreviation {
        fn from(sys: html_sys::text::Abbreviation) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Abbreviation` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub enum AbbreviationChild {
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
    impl std::convert::From<crate::generated::all::Abbreviation> for AbbreviationChild {
        fn from(value: crate::generated::all::Abbreviation) -> Self {
            Self::Abbreviation(value)
        }
    }
    impl std::convert::From<crate::generated::all::Address> for AbbreviationChild {
        fn from(value: crate::generated::all::Address) -> Self {
            Self::Address(value)
        }
    }
    impl std::convert::From<crate::generated::all::Anchor> for AbbreviationChild {
        fn from(value: crate::generated::all::Anchor) -> Self {
            Self::Anchor(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalIsolate>
    for AbbreviationChild {
        fn from(value: crate::generated::all::BidirectionalIsolate) -> Self {
            Self::BidirectionalIsolate(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalTextOverride>
    for AbbreviationChild {
        fn from(value: crate::generated::all::BidirectionalTextOverride) -> Self {
            Self::BidirectionalTextOverride(value)
        }
    }
    impl std::convert::From<crate::generated::all::BlockQuote> for AbbreviationChild {
        fn from(value: crate::generated::all::BlockQuote) -> Self {
            Self::BlockQuote(value)
        }
    }
    impl std::convert::From<crate::generated::all::Bold> for AbbreviationChild {
        fn from(value: crate::generated::all::Bold) -> Self {
            Self::Bold(value)
        }
    }
    impl std::convert::From<crate::generated::all::Button> for AbbreviationChild {
        fn from(value: crate::generated::all::Button) -> Self {
            Self::Button(value)
        }
    }
    impl std::convert::From<crate::generated::all::Cite> for AbbreviationChild {
        fn from(value: crate::generated::all::Cite) -> Self {
            Self::Cite(value)
        }
    }
    impl std::convert::From<crate::generated::all::Code> for AbbreviationChild {
        fn from(value: crate::generated::all::Code) -> Self {
            Self::Code(value)
        }
    }
    impl std::convert::From<crate::generated::all::Data> for AbbreviationChild {
        fn from(value: crate::generated::all::Data) -> Self {
            Self::Data(value)
        }
    }
    impl std::convert::From<crate::generated::all::DataList> for AbbreviationChild {
        fn from(value: crate::generated::all::DataList) -> Self {
            Self::DataList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Definition> for AbbreviationChild {
        fn from(value: crate::generated::all::Definition) -> Self {
            Self::Definition(value)
        }
    }
    impl std::convert::From<crate::generated::all::DeletedText> for AbbreviationChild {
        fn from(value: crate::generated::all::DeletedText) -> Self {
            Self::DeletedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::DescriptionList>
    for AbbreviationChild {
        fn from(value: crate::generated::all::DescriptionList) -> Self {
            Self::DescriptionList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Details> for AbbreviationChild {
        fn from(value: crate::generated::all::Details) -> Self {
            Self::Details(value)
        }
    }
    impl std::convert::From<crate::generated::all::Dialog> for AbbreviationChild {
        fn from(value: crate::generated::all::Dialog) -> Self {
            Self::Dialog(value)
        }
    }
    impl std::convert::From<crate::generated::all::Division> for AbbreviationChild {
        fn from(value: crate::generated::all::Division) -> Self {
            Self::Division(value)
        }
    }
    impl std::convert::From<crate::generated::all::Emphasis> for AbbreviationChild {
        fn from(value: crate::generated::all::Emphasis) -> Self {
            Self::Emphasis(value)
        }
    }
    impl std::convert::From<crate::generated::all::Fieldset> for AbbreviationChild {
        fn from(value: crate::generated::all::Fieldset) -> Self {
            Self::Fieldset(value)
        }
    }
    impl std::convert::From<crate::generated::all::Figure> for AbbreviationChild {
        fn from(value: crate::generated::all::Figure) -> Self {
            Self::Figure(value)
        }
    }
    impl std::convert::From<crate::generated::all::Footer> for AbbreviationChild {
        fn from(value: crate::generated::all::Footer) -> Self {
            Self::Footer(value)
        }
    }
    impl std::convert::From<crate::generated::all::Form> for AbbreviationChild {
        fn from(value: crate::generated::all::Form) -> Self {
            Self::Form(value)
        }
    }
    impl std::convert::From<crate::generated::all::Header> for AbbreviationChild {
        fn from(value: crate::generated::all::Header) -> Self {
            Self::Header(value)
        }
    }
    impl std::convert::From<crate::generated::all::ImageMap> for AbbreviationChild {
        fn from(value: crate::generated::all::ImageMap) -> Self {
            Self::ImageMap(value)
        }
    }
    impl std::convert::From<crate::generated::all::Input> for AbbreviationChild {
        fn from(value: crate::generated::all::Input) -> Self {
            Self::Input(value)
        }
    }
    impl std::convert::From<crate::generated::all::InsertedText> for AbbreviationChild {
        fn from(value: crate::generated::all::InsertedText) -> Self {
            Self::InsertedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Italic> for AbbreviationChild {
        fn from(value: crate::generated::all::Italic) -> Self {
            Self::Italic(value)
        }
    }
    impl std::convert::From<crate::generated::all::KeyboardInput> for AbbreviationChild {
        fn from(value: crate::generated::all::KeyboardInput) -> Self {
            Self::KeyboardInput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Label> for AbbreviationChild {
        fn from(value: crate::generated::all::Label) -> Self {
            Self::Label(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreak> for AbbreviationChild {
        fn from(value: crate::generated::all::LineBreak) -> Self {
            Self::LineBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreakOpportunity>
    for AbbreviationChild {
        fn from(value: crate::generated::all::LineBreakOpportunity) -> Self {
            Self::LineBreakOpportunity(value)
        }
    }
    impl std::convert::From<crate::generated::all::MarkText> for AbbreviationChild {
        fn from(value: crate::generated::all::MarkText) -> Self {
            Self::MarkText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Menu> for AbbreviationChild {
        fn from(value: crate::generated::all::Menu) -> Self {
            Self::Menu(value)
        }
    }
    impl std::convert::From<crate::generated::all::Meter> for AbbreviationChild {
        fn from(value: crate::generated::all::Meter) -> Self {
            Self::Meter(value)
        }
    }
    impl std::convert::From<crate::generated::all::OrderedList> for AbbreviationChild {
        fn from(value: crate::generated::all::OrderedList) -> Self {
            Self::OrderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Output> for AbbreviationChild {
        fn from(value: crate::generated::all::Output) -> Self {
            Self::Output(value)
        }
    }
    impl std::convert::From<crate::generated::all::Paragraph> for AbbreviationChild {
        fn from(value: crate::generated::all::Paragraph) -> Self {
            Self::Paragraph(value)
        }
    }
    impl std::convert::From<crate::generated::all::PreformattedText>
    for AbbreviationChild {
        fn from(value: crate::generated::all::PreformattedText) -> Self {
            Self::PreformattedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Progress> for AbbreviationChild {
        fn from(value: crate::generated::all::Progress) -> Self {
            Self::Progress(value)
        }
    }
    impl std::convert::From<crate::generated::all::Quotation> for AbbreviationChild {
        fn from(value: crate::generated::all::Quotation) -> Self {
            Self::Quotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::RubyAnnotation>
    for AbbreviationChild {
        fn from(value: crate::generated::all::RubyAnnotation) -> Self {
            Self::RubyAnnotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::SampleOutput> for AbbreviationChild {
        fn from(value: crate::generated::all::SampleOutput) -> Self {
            Self::SampleOutput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for AbbreviationChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Select> for AbbreviationChild {
        fn from(value: crate::generated::all::Select) -> Self {
            Self::Select(value)
        }
    }
    impl std::convert::From<crate::generated::all::SideComment> for AbbreviationChild {
        fn from(value: crate::generated::all::SideComment) -> Self {
            Self::SideComment(value)
        }
    }
    impl std::convert::From<crate::generated::all::Slot> for AbbreviationChild {
        fn from(value: crate::generated::all::Slot) -> Self {
            Self::Slot(value)
        }
    }
    impl std::convert::From<crate::generated::all::Span> for AbbreviationChild {
        fn from(value: crate::generated::all::Span) -> Self {
            Self::Span(value)
        }
    }
    impl std::convert::From<crate::generated::all::StrikeThrough> for AbbreviationChild {
        fn from(value: crate::generated::all::StrikeThrough) -> Self {
            Self::StrikeThrough(value)
        }
    }
    impl std::convert::From<crate::generated::all::Strong> for AbbreviationChild {
        fn from(value: crate::generated::all::Strong) -> Self {
            Self::Strong(value)
        }
    }
    impl std::convert::From<crate::generated::all::SubScript> for AbbreviationChild {
        fn from(value: crate::generated::all::SubScript) -> Self {
            Self::SubScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::SuperScript> for AbbreviationChild {
        fn from(value: crate::generated::all::SuperScript) -> Self {
            Self::SuperScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::Table> for AbbreviationChild {
        fn from(value: crate::generated::all::Table) -> Self {
            Self::Table(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for AbbreviationChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::convert::From<crate::generated::all::TextArea> for AbbreviationChild {
        fn from(value: crate::generated::all::TextArea) -> Self {
            Self::TextArea(value)
        }
    }
    impl std::convert::From<crate::generated::all::ThematicBreak> for AbbreviationChild {
        fn from(value: crate::generated::all::ThematicBreak) -> Self {
            Self::ThematicBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::Time> for AbbreviationChild {
        fn from(value: crate::generated::all::Time) -> Self {
            Self::Time(value)
        }
    }
    impl std::convert::From<crate::generated::all::Underline> for AbbreviationChild {
        fn from(value: crate::generated::all::Underline) -> Self {
            Self::Underline(value)
        }
    }
    impl std::convert::From<crate::generated::all::UnorderedList> for AbbreviationChild {
        fn from(value: crate::generated::all::UnorderedList) -> Self {
            Self::UnorderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Variable> for AbbreviationChild {
        fn from(value: crate::generated::all::Variable) -> Self {
            Self::Variable(value)
        }
    }
    impl std::fmt::Display for AbbreviationChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Abbreviation(el) => write!(f, "{el}"),
                Self::Address(el) => write!(f, "{el}"),
                Self::Anchor(el) => write!(f, "{el}"),
                Self::BidirectionalIsolate(el) => write!(f, "{el}"),
                Self::BidirectionalTextOverride(el) => write!(f, "{el}"),
                Self::BlockQuote(el) => write!(f, "{el}"),
                Self::Bold(el) => write!(f, "{el}"),
                Self::Button(el) => write!(f, "{el}"),
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
                Self::Emphasis(el) => write!(f, "{el}"),
                Self::Fieldset(el) => write!(f, "{el}"),
                Self::Figure(el) => write!(f, "{el}"),
                Self::Footer(el) => write!(f, "{el}"),
                Self::Form(el) => write!(f, "{el}"),
                Self::Header(el) => write!(f, "{el}"),
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
                Self::OrderedList(el) => write!(f, "{el}"),
                Self::Output(el) => write!(f, "{el}"),
                Self::Paragraph(el) => write!(f, "{el}"),
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
            }
        }
    }
}
