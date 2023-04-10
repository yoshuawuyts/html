pub mod element {
    /// The HTML `<button>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    #[doc(alias = "button")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Button {
        sys: html_sys::forms::Button,
        children: Vec<super::child::ButtonChild>,
    }
    impl Button {
        /// Create a new builder
        pub fn builder() -> super::builder::ButtonBuilder {
            super::builder::ButtonBuilder::new(Default::default())
        }
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
        pub fn set_form(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.form = value.map(|v| v.into());
        }
        /// Get the value of the `formaction` attribute
        pub fn form_action(&self) -> std::option::Option<&str> {
            self.sys.form_action.as_deref()
        }
        /// Set the value of the `formaction` attribute
        pub fn set_form_action(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.form_action = value.map(|v| v.into());
        }
        /// Get the value of the `formenctype` attribute
        pub fn form_enctype(&self) -> std::option::Option<&str> {
            self.sys.form_enctype.as_deref()
        }
        /// Set the value of the `formenctype` attribute
        pub fn set_form_enctype(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.form_enctype = value.map(|v| v.into());
        }
        /// Get the value of the `formmethod` attribute
        pub fn form_method(&self) -> std::option::Option<&str> {
            self.sys.form_method.as_deref()
        }
        /// Set the value of the `formmethod` attribute
        pub fn set_form_method(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.form_method = value.map(|v| v.into());
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
        pub fn set_form_target(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.form_target = value.map(|v| v.into());
        }
        /// Get the value of the `name` attribute
        pub fn name(&self) -> std::option::Option<&str> {
            self.sys.name.as_deref()
        }
        /// Set the value of the `name` attribute
        pub fn set_name(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.name = value.map(|v| v.into());
        }
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.type_ = value.map(|v| v.into());
        }
        /// Get the value of the `value` attribute
        pub fn value(&self) -> std::option::Option<&str> {
            self.sys.value.as_deref()
        }
        /// Set the value of the `value` attribute
        pub fn set_value(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.value = value.map(|v| v.into());
        }
        /// Get the value of the `accesskey` attribute
        pub fn access_key(&self) -> std::option::Option<&str> {
            self.sys.access_key.as_deref()
        }
        /// Set the value of the `accesskey` attribute
        pub fn set_access_key(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.access_key = value.map(|v| v.into());
        }
        /// Get the value of the `autocapitalize` attribute
        pub fn auto_capitalize(&self) -> std::option::Option<&str> {
            self.sys.auto_capitalize.as_deref()
        }
        /// Set the value of the `autocapitalize` attribute
        pub fn set_auto_capitalize(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.auto_capitalize = value.map(|v| v.into());
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
        pub fn set_class_(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.class_ = value.map(|v| v.into());
        }
        /// Get the value of the `contenteditable` attribute
        pub fn content_editable(&self) -> std::option::Option<&str> {
            self.sys.content_editable.as_deref()
        }
        /// Set the value of the `contenteditable` attribute
        pub fn set_content_editable(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.content_editable = value.map(|v| v.into());
        }
        /// Get the value of the `dir` attribute
        pub fn direction(&self) -> std::option::Option<&str> {
            self.sys.direction.as_deref()
        }
        /// Set the value of the `dir` attribute
        pub fn set_direction(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.direction = value.map(|v| v.into());
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
        pub fn set_enter_key_hint(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.enter_key_hint = value.map(|v| v.into());
        }
        /// Get the value of the `exportparts` attribute
        pub fn export_parts(&self) -> std::option::Option<&str> {
            self.sys.export_parts.as_deref()
        }
        /// Set the value of the `exportparts` attribute
        pub fn set_export_parts(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.export_parts = value.map(|v| v.into());
        }
        /// Get the value of the `hidden` attribute
        pub fn hidden(&self) -> std::option::Option<&str> {
            self.sys.hidden.as_deref()
        }
        /// Set the value of the `hidden` attribute
        pub fn set_hidden(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.hidden = value.map(|v| v.into());
        }
        /// Get the value of the `id` attribute
        pub fn id(&self) -> std::option::Option<&str> {
            self.sys.id.as_deref()
        }
        /// Set the value of the `id` attribute
        pub fn set_id(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.id = value.map(|v| v.into());
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
        pub fn set_input_mode(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.input_mode = value.map(|v| v.into());
        }
        /// Get the value of the `is` attribute
        pub fn is_(&self) -> std::option::Option<&str> {
            self.sys.is_.as_deref()
        }
        /// Set the value of the `is` attribute
        pub fn set_is_(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.is_ = value.map(|v| v.into());
        }
        /// Get the value of the `itemid` attribute
        pub fn item_id(&self) -> std::option::Option<&str> {
            self.sys.item_id.as_deref()
        }
        /// Set the value of the `itemid` attribute
        pub fn set_item_id(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.item_id = value.map(|v| v.into());
        }
        /// Get the value of the `itemprop` attribute
        pub fn item_prop(&self) -> std::option::Option<&str> {
            self.sys.item_prop.as_deref()
        }
        /// Set the value of the `itemprop` attribute
        pub fn set_item_prop(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.item_prop = value.map(|v| v.into());
        }
        /// Get the value of the `itemref` attribute
        pub fn item_ref(&self) -> std::option::Option<&str> {
            self.sys.item_ref.as_deref()
        }
        /// Set the value of the `itemref` attribute
        pub fn set_item_ref(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.item_ref = value.map(|v| v.into());
        }
        /// Get the value of the `itemscope` attribute
        pub fn item_scope(&self) -> std::option::Option<&str> {
            self.sys.item_scope.as_deref()
        }
        /// Set the value of the `itemscope` attribute
        pub fn set_item_scope(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.item_scope = value.map(|v| v.into());
        }
        /// Get the value of the `itemtype` attribute
        pub fn item_type(&self) -> std::option::Option<&str> {
            self.sys.item_type.as_deref()
        }
        /// Set the value of the `itemtype` attribute
        pub fn set_item_type(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.item_type = value.map(|v| v.into());
        }
        /// Get the value of the `lang` attribute
        pub fn lang(&self) -> std::option::Option<&str> {
            self.sys.lang.as_deref()
        }
        /// Set the value of the `lang` attribute
        pub fn set_lang(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.lang = value.map(|v| v.into());
        }
        /// Get the value of the `nonce` attribute
        pub fn nonce(&self) -> std::option::Option<&str> {
            self.sys.nonce.as_deref()
        }
        /// Set the value of the `nonce` attribute
        pub fn set_nonce(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.nonce = value.map(|v| v.into());
        }
        /// Get the value of the `part` attribute
        pub fn part(&self) -> std::option::Option<&str> {
            self.sys.part.as_deref()
        }
        /// Set the value of the `part` attribute
        pub fn set_part(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.part = value.map(|v| v.into());
        }
        /// Get the value of the `slot` attribute
        pub fn slot(&self) -> std::option::Option<&str> {
            self.sys.slot.as_deref()
        }
        /// Set the value of the `slot` attribute
        pub fn set_slot(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.slot = value.map(|v| v.into());
        }
        /// Get the value of the `spellcheck` attribute
        pub fn spellcheck(&self) -> std::option::Option<&str> {
            self.sys.spellcheck.as_deref()
        }
        /// Set the value of the `spellcheck` attribute
        pub fn set_spellcheck(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.spellcheck = value.map(|v| v.into());
        }
        /// Get the value of the `style` attribute
        pub fn style(&self) -> std::option::Option<&str> {
            self.sys.style.as_deref()
        }
        /// Set the value of the `style` attribute
        pub fn set_style(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.style = value.map(|v| v.into());
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
        pub fn set_title(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.title = value.map(|v| v.into());
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
    impl Button {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::ButtonChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::ButtonChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Button {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
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
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Button` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub enum ButtonChild {
        /// The Abbreviation element
        Abbreviation(crate::generated::all::Abbreviation),
        /// The Anchor element
        Anchor(crate::generated::all::Anchor),
        /// The BidirectionalIsolate element
        BidirectionalIsolate(crate::generated::all::BidirectionalIsolate),
        /// The BidirectionalTextOverride element
        BidirectionalTextOverride(crate::generated::all::BidirectionalTextOverride),
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
        /// The Emphasis element
        Emphasis(crate::generated::all::Emphasis),
        /// The ImageMap element
        ImageMap(crate::generated::all::ImageMap),
        /// The ImageMapArea element
        ImageMapArea(crate::generated::all::ImageMapArea),
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
        /// The Meter element
        Meter(crate::generated::all::Meter),
        /// The NoScript element
        NoScript(crate::generated::all::NoScript),
        /// The Output element
        Output(crate::generated::all::Output),
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
        /// The Template element
        Template(crate::generated::all::Template),
        /// The TextArea element
        TextArea(crate::generated::all::TextArea),
        /// The Time element
        Time(crate::generated::all::Time),
        /// The Underline element
        Underline(crate::generated::all::Underline),
        /// The Variable element
        Variable(crate::generated::all::Variable),
        /// The Text element
        Text(std::borrow::Cow<'static, str>),
    }
    impl std::convert::From<crate::generated::all::Abbreviation> for ButtonChild {
        fn from(value: crate::generated::all::Abbreviation) -> Self {
            Self::Abbreviation(value)
        }
    }
    impl std::convert::From<crate::generated::all::Anchor> for ButtonChild {
        fn from(value: crate::generated::all::Anchor) -> Self {
            Self::Anchor(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalIsolate>
    for ButtonChild {
        fn from(value: crate::generated::all::BidirectionalIsolate) -> Self {
            Self::BidirectionalIsolate(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalTextOverride>
    for ButtonChild {
        fn from(value: crate::generated::all::BidirectionalTextOverride) -> Self {
            Self::BidirectionalTextOverride(value)
        }
    }
    impl std::convert::From<crate::generated::all::Bold> for ButtonChild {
        fn from(value: crate::generated::all::Bold) -> Self {
            Self::Bold(value)
        }
    }
    impl std::convert::From<crate::generated::all::Button> for ButtonChild {
        fn from(value: crate::generated::all::Button) -> Self {
            Self::Button(value)
        }
    }
    impl std::convert::From<crate::generated::all::Cite> for ButtonChild {
        fn from(value: crate::generated::all::Cite) -> Self {
            Self::Cite(value)
        }
    }
    impl std::convert::From<crate::generated::all::Code> for ButtonChild {
        fn from(value: crate::generated::all::Code) -> Self {
            Self::Code(value)
        }
    }
    impl std::convert::From<crate::generated::all::Data> for ButtonChild {
        fn from(value: crate::generated::all::Data) -> Self {
            Self::Data(value)
        }
    }
    impl std::convert::From<crate::generated::all::DataList> for ButtonChild {
        fn from(value: crate::generated::all::DataList) -> Self {
            Self::DataList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Definition> for ButtonChild {
        fn from(value: crate::generated::all::Definition) -> Self {
            Self::Definition(value)
        }
    }
    impl std::convert::From<crate::generated::all::DeletedText> for ButtonChild {
        fn from(value: crate::generated::all::DeletedText) -> Self {
            Self::DeletedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Emphasis> for ButtonChild {
        fn from(value: crate::generated::all::Emphasis) -> Self {
            Self::Emphasis(value)
        }
    }
    impl std::convert::From<crate::generated::all::ImageMap> for ButtonChild {
        fn from(value: crate::generated::all::ImageMap) -> Self {
            Self::ImageMap(value)
        }
    }
    impl std::convert::From<crate::generated::all::ImageMapArea> for ButtonChild {
        fn from(value: crate::generated::all::ImageMapArea) -> Self {
            Self::ImageMapArea(value)
        }
    }
    impl std::convert::From<crate::generated::all::Input> for ButtonChild {
        fn from(value: crate::generated::all::Input) -> Self {
            Self::Input(value)
        }
    }
    impl std::convert::From<crate::generated::all::InsertedText> for ButtonChild {
        fn from(value: crate::generated::all::InsertedText) -> Self {
            Self::InsertedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Italic> for ButtonChild {
        fn from(value: crate::generated::all::Italic) -> Self {
            Self::Italic(value)
        }
    }
    impl std::convert::From<crate::generated::all::KeyboardInput> for ButtonChild {
        fn from(value: crate::generated::all::KeyboardInput) -> Self {
            Self::KeyboardInput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Label> for ButtonChild {
        fn from(value: crate::generated::all::Label) -> Self {
            Self::Label(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreak> for ButtonChild {
        fn from(value: crate::generated::all::LineBreak) -> Self {
            Self::LineBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreakOpportunity>
    for ButtonChild {
        fn from(value: crate::generated::all::LineBreakOpportunity) -> Self {
            Self::LineBreakOpportunity(value)
        }
    }
    impl std::convert::From<crate::generated::all::MarkText> for ButtonChild {
        fn from(value: crate::generated::all::MarkText) -> Self {
            Self::MarkText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Meter> for ButtonChild {
        fn from(value: crate::generated::all::Meter) -> Self {
            Self::Meter(value)
        }
    }
    impl std::convert::From<crate::generated::all::NoScript> for ButtonChild {
        fn from(value: crate::generated::all::NoScript) -> Self {
            Self::NoScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::Output> for ButtonChild {
        fn from(value: crate::generated::all::Output) -> Self {
            Self::Output(value)
        }
    }
    impl std::convert::From<crate::generated::all::Progress> for ButtonChild {
        fn from(value: crate::generated::all::Progress) -> Self {
            Self::Progress(value)
        }
    }
    impl std::convert::From<crate::generated::all::Quotation> for ButtonChild {
        fn from(value: crate::generated::all::Quotation) -> Self {
            Self::Quotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::RubyAnnotation> for ButtonChild {
        fn from(value: crate::generated::all::RubyAnnotation) -> Self {
            Self::RubyAnnotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::SampleOutput> for ButtonChild {
        fn from(value: crate::generated::all::SampleOutput) -> Self {
            Self::SampleOutput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for ButtonChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Select> for ButtonChild {
        fn from(value: crate::generated::all::Select) -> Self {
            Self::Select(value)
        }
    }
    impl std::convert::From<crate::generated::all::SideComment> for ButtonChild {
        fn from(value: crate::generated::all::SideComment) -> Self {
            Self::SideComment(value)
        }
    }
    impl std::convert::From<crate::generated::all::Slot> for ButtonChild {
        fn from(value: crate::generated::all::Slot) -> Self {
            Self::Slot(value)
        }
    }
    impl std::convert::From<crate::generated::all::Span> for ButtonChild {
        fn from(value: crate::generated::all::Span) -> Self {
            Self::Span(value)
        }
    }
    impl std::convert::From<crate::generated::all::StrikeThrough> for ButtonChild {
        fn from(value: crate::generated::all::StrikeThrough) -> Self {
            Self::StrikeThrough(value)
        }
    }
    impl std::convert::From<crate::generated::all::Strong> for ButtonChild {
        fn from(value: crate::generated::all::Strong) -> Self {
            Self::Strong(value)
        }
    }
    impl std::convert::From<crate::generated::all::SubScript> for ButtonChild {
        fn from(value: crate::generated::all::SubScript) -> Self {
            Self::SubScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::SuperScript> for ButtonChild {
        fn from(value: crate::generated::all::SuperScript) -> Self {
            Self::SuperScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for ButtonChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::convert::From<crate::generated::all::TextArea> for ButtonChild {
        fn from(value: crate::generated::all::TextArea) -> Self {
            Self::TextArea(value)
        }
    }
    impl std::convert::From<crate::generated::all::Time> for ButtonChild {
        fn from(value: crate::generated::all::Time) -> Self {
            Self::Time(value)
        }
    }
    impl std::convert::From<crate::generated::all::Underline> for ButtonChild {
        fn from(value: crate::generated::all::Underline) -> Self {
            Self::Underline(value)
        }
    }
    impl std::convert::From<crate::generated::all::Variable> for ButtonChild {
        fn from(value: crate::generated::all::Variable) -> Self {
            Self::Variable(value)
        }
    }
    impl std::convert::From<std::borrow::Cow<'static, str>> for ButtonChild {
        fn from(value: std::borrow::Cow<'static, str>) -> Self {
            Self::Text(value)
        }
    }
    impl std::convert::From<&'static str> for ButtonChild {
        fn from(value: &'static str) -> Self {
            Self::Text(value.into())
        }
    }
    impl std::convert::From<String> for ButtonChild {
        fn from(value: String) -> Self {
            Self::Text(value.into())
        }
    }
    impl std::fmt::Display for ButtonChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Abbreviation(el) => write!(f, "{el}"),
                Self::Anchor(el) => write!(f, "{el}"),
                Self::BidirectionalIsolate(el) => write!(f, "{el}"),
                Self::BidirectionalTextOverride(el) => write!(f, "{el}"),
                Self::Bold(el) => write!(f, "{el}"),
                Self::Button(el) => write!(f, "{el}"),
                Self::Cite(el) => write!(f, "{el}"),
                Self::Code(el) => write!(f, "{el}"),
                Self::Data(el) => write!(f, "{el}"),
                Self::DataList(el) => write!(f, "{el}"),
                Self::Definition(el) => write!(f, "{el}"),
                Self::DeletedText(el) => write!(f, "{el}"),
                Self::Emphasis(el) => write!(f, "{el}"),
                Self::ImageMap(el) => write!(f, "{el}"),
                Self::ImageMapArea(el) => write!(f, "{el}"),
                Self::Input(el) => write!(f, "{el}"),
                Self::InsertedText(el) => write!(f, "{el}"),
                Self::Italic(el) => write!(f, "{el}"),
                Self::KeyboardInput(el) => write!(f, "{el}"),
                Self::Label(el) => write!(f, "{el}"),
                Self::LineBreak(el) => write!(f, "{el}"),
                Self::LineBreakOpportunity(el) => write!(f, "{el}"),
                Self::MarkText(el) => write!(f, "{el}"),
                Self::Meter(el) => write!(f, "{el}"),
                Self::NoScript(el) => write!(f, "{el}"),
                Self::Output(el) => write!(f, "{el}"),
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
                Self::Template(el) => write!(f, "{el}"),
                Self::TextArea(el) => write!(f, "{el}"),
                Self::Time(el) => write!(f, "{el}"),
                Self::Underline(el) => write!(f, "{el}"),
                Self::Variable(el) => write!(f, "{el}"),
                Self::Text(el) => write!(f, "{el}"),
            }
        }
    }
}
pub mod builder {
    /// A builder struct for Button
    pub struct ButtonBuilder {
        element: super::element::Button,
    }
    impl ButtonBuilder {
        pub(crate) fn new(element: super::element::Button) -> Self {
            Self { element }
        }
        /// Finish building the element
        pub fn build(&mut self) -> super::element::Button {
            self.element.clone()
        }
        /// Append a new `Abbreviation` element
        pub fn abbreviation<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::AbbreviationBuilder),
        {
            let ty: crate::generated::all::Abbreviation = Default::default();
            let mut ty_builder = crate::generated::all::builders::AbbreviationBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Anchor` element
        pub fn anchor<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::AnchorBuilder),
        {
            let ty: crate::generated::all::Anchor = Default::default();
            let mut ty_builder = crate::generated::all::builders::AnchorBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `BidirectionalIsolate` element
        pub fn bidirectional_isolate<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::BidirectionalIsolateBuilder),
        {
            let ty: crate::generated::all::BidirectionalIsolate = Default::default();
            let mut ty_builder = crate::generated::all::builders::BidirectionalIsolateBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `BidirectionalTextOverride` element
        pub fn bidirectional_text_override<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(
                &mut crate::generated::all::builders::BidirectionalTextOverrideBuilder,
            ),
        {
            let ty: crate::generated::all::BidirectionalTextOverride = Default::default();
            let mut ty_builder = crate::generated::all::builders::BidirectionalTextOverrideBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Bold` element
        pub fn bold<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::BoldBuilder),
        {
            let ty: crate::generated::all::Bold = Default::default();
            let mut ty_builder = crate::generated::all::builders::BoldBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Button` element
        pub fn button<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ButtonBuilder),
        {
            let ty: crate::generated::all::Button = Default::default();
            let mut ty_builder = crate::generated::all::builders::ButtonBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Cite` element
        pub fn cite<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::CiteBuilder),
        {
            let ty: crate::generated::all::Cite = Default::default();
            let mut ty_builder = crate::generated::all::builders::CiteBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Code` element
        pub fn code<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::CodeBuilder),
        {
            let ty: crate::generated::all::Code = Default::default();
            let mut ty_builder = crate::generated::all::builders::CodeBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Data` element
        pub fn data<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DataBuilder),
        {
            let ty: crate::generated::all::Data = Default::default();
            let mut ty_builder = crate::generated::all::builders::DataBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `DataList` element
        pub fn data_list<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DataListBuilder),
        {
            let ty: crate::generated::all::DataList = Default::default();
            let mut ty_builder = crate::generated::all::builders::DataListBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Definition` element
        pub fn definition<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DefinitionBuilder),
        {
            let ty: crate::generated::all::Definition = Default::default();
            let mut ty_builder = crate::generated::all::builders::DefinitionBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `DeletedText` element
        pub fn deleted_text<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DeletedTextBuilder),
        {
            let ty: crate::generated::all::DeletedText = Default::default();
            let mut ty_builder = crate::generated::all::builders::DeletedTextBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Emphasis` element
        pub fn emphasis<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::EmphasisBuilder),
        {
            let ty: crate::generated::all::Emphasis = Default::default();
            let mut ty_builder = crate::generated::all::builders::EmphasisBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `ImageMap` element
        pub fn image_map<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ImageMapBuilder),
        {
            let ty: crate::generated::all::ImageMap = Default::default();
            let mut ty_builder = crate::generated::all::builders::ImageMapBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `ImageMapArea` element
        pub fn image_map_area<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ImageMapAreaBuilder),
        {
            let ty: crate::generated::all::ImageMapArea = Default::default();
            let mut ty_builder = crate::generated::all::builders::ImageMapAreaBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Input` element
        pub fn input<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::InputBuilder),
        {
            let ty: crate::generated::all::Input = Default::default();
            let mut ty_builder = crate::generated::all::builders::InputBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `InsertedText` element
        pub fn inserted_text<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::InsertedTextBuilder),
        {
            let ty: crate::generated::all::InsertedText = Default::default();
            let mut ty_builder = crate::generated::all::builders::InsertedTextBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Italic` element
        pub fn italic<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ItalicBuilder),
        {
            let ty: crate::generated::all::Italic = Default::default();
            let mut ty_builder = crate::generated::all::builders::ItalicBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `KeyboardInput` element
        pub fn keyboard_input<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::KeyboardInputBuilder),
        {
            let ty: crate::generated::all::KeyboardInput = Default::default();
            let mut ty_builder = crate::generated::all::builders::KeyboardInputBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Label` element
        pub fn label<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::LabelBuilder),
        {
            let ty: crate::generated::all::Label = Default::default();
            let mut ty_builder = crate::generated::all::builders::LabelBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `LineBreak` element
        pub fn line_break<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::LineBreakBuilder),
        {
            let ty: crate::generated::all::LineBreak = Default::default();
            let mut ty_builder = crate::generated::all::builders::LineBreakBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `LineBreakOpportunity` element
        pub fn line_break_opportunity<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::LineBreakOpportunityBuilder),
        {
            let ty: crate::generated::all::LineBreakOpportunity = Default::default();
            let mut ty_builder = crate::generated::all::builders::LineBreakOpportunityBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `MarkText` element
        pub fn mark_text<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::MarkTextBuilder),
        {
            let ty: crate::generated::all::MarkText = Default::default();
            let mut ty_builder = crate::generated::all::builders::MarkTextBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Meter` element
        pub fn meter<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::MeterBuilder),
        {
            let ty: crate::generated::all::Meter = Default::default();
            let mut ty_builder = crate::generated::all::builders::MeterBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `NoScript` element
        pub fn no_script<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::NoScriptBuilder),
        {
            let ty: crate::generated::all::NoScript = Default::default();
            let mut ty_builder = crate::generated::all::builders::NoScriptBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Output` element
        pub fn output<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::OutputBuilder),
        {
            let ty: crate::generated::all::Output = Default::default();
            let mut ty_builder = crate::generated::all::builders::OutputBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Progress` element
        pub fn progress<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ProgressBuilder),
        {
            let ty: crate::generated::all::Progress = Default::default();
            let mut ty_builder = crate::generated::all::builders::ProgressBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Quotation` element
        pub fn quotation<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::QuotationBuilder),
        {
            let ty: crate::generated::all::Quotation = Default::default();
            let mut ty_builder = crate::generated::all::builders::QuotationBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `RubyAnnotation` element
        pub fn ruby_annotation<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::RubyAnnotationBuilder),
        {
            let ty: crate::generated::all::RubyAnnotation = Default::default();
            let mut ty_builder = crate::generated::all::builders::RubyAnnotationBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `SampleOutput` element
        pub fn sample_output<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SampleOutputBuilder),
        {
            let ty: crate::generated::all::SampleOutput = Default::default();
            let mut ty_builder = crate::generated::all::builders::SampleOutputBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Script` element
        pub fn script<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ScriptBuilder),
        {
            let ty: crate::generated::all::Script = Default::default();
            let mut ty_builder = crate::generated::all::builders::ScriptBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Select` element
        pub fn select<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SelectBuilder),
        {
            let ty: crate::generated::all::Select = Default::default();
            let mut ty_builder = crate::generated::all::builders::SelectBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `SideComment` element
        pub fn side_comment<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SideCommentBuilder),
        {
            let ty: crate::generated::all::SideComment = Default::default();
            let mut ty_builder = crate::generated::all::builders::SideCommentBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Slot` element
        pub fn slot<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SlotBuilder),
        {
            let ty: crate::generated::all::Slot = Default::default();
            let mut ty_builder = crate::generated::all::builders::SlotBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Span` element
        pub fn span<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SpanBuilder),
        {
            let ty: crate::generated::all::Span = Default::default();
            let mut ty_builder = crate::generated::all::builders::SpanBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `StrikeThrough` element
        pub fn strike_through<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::StrikeThroughBuilder),
        {
            let ty: crate::generated::all::StrikeThrough = Default::default();
            let mut ty_builder = crate::generated::all::builders::StrikeThroughBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Strong` element
        pub fn strong<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::StrongBuilder),
        {
            let ty: crate::generated::all::Strong = Default::default();
            let mut ty_builder = crate::generated::all::builders::StrongBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `SubScript` element
        pub fn sub_script<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SubScriptBuilder),
        {
            let ty: crate::generated::all::SubScript = Default::default();
            let mut ty_builder = crate::generated::all::builders::SubScriptBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `SuperScript` element
        pub fn super_script<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SuperScriptBuilder),
        {
            let ty: crate::generated::all::SuperScript = Default::default();
            let mut ty_builder = crate::generated::all::builders::SuperScriptBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Template` element
        pub fn template<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::TemplateBuilder),
        {
            let ty: crate::generated::all::Template = Default::default();
            let mut ty_builder = crate::generated::all::builders::TemplateBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `TextArea` element
        pub fn text_area<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::TextAreaBuilder),
        {
            let ty: crate::generated::all::TextArea = Default::default();
            let mut ty_builder = crate::generated::all::builders::TextAreaBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Time` element
        pub fn time<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::TimeBuilder),
        {
            let ty: crate::generated::all::Time = Default::default();
            let mut ty_builder = crate::generated::all::builders::TimeBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Underline` element
        pub fn underline<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::UnderlineBuilder),
        {
            let ty: crate::generated::all::Underline = Default::default();
            let mut ty_builder = crate::generated::all::builders::UnderlineBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Variable` element
        pub fn variable<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::VariableBuilder),
        {
            let ty: crate::generated::all::Variable = Default::default();
            let mut ty_builder = crate::generated::all::builders::VariableBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new text element.
        pub fn text(
            &mut self,
            s: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            let cow = s.into();
            self.element.children_mut().push(cow.into());
            self
        }
    }
}
