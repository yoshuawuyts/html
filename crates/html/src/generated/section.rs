pub mod element {
    /// The HTML `<section>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
    #[doc(alias = "section")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct Section {
        sys: html_sys::sections::Section,
        children: Vec<super::child::SectionChild>,
    }
    impl Section {
        /// Create a new builder
        pub fn builder() -> super::builder::SectionBuilder {
            super::builder::SectionBuilder::new(Default::default())
        }
    }
    impl Section {
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
    impl Section {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::SectionChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::SectionChild> {
            &mut self.children
        }
    }
    impl std::fmt::Display for Section {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                std::fmt::Display::fmt(&el, f)?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Section {}
    impl crate::FlowContent for Section {}
    impl crate::SectioningContent for Section {}
    impl crate::PalpableContent for Section {}
    impl std::convert::Into<html_sys::sections::Section> for Section {
        fn into(self) -> html_sys::sections::Section {
            self.sys
        }
    }
    impl From<html_sys::sections::Section> for Section {
        fn from(sys: html_sys::sections::Section) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Section` element
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub enum SectionChild {
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
        /// The Main element
        Main(crate::generated::all::Main),
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
    impl std::convert::From<crate::generated::all::Address> for SectionChild {
        fn from(value: crate::generated::all::Address) -> Self {
            Self::Address(value)
        }
    }
    impl std::convert::From<crate::generated::all::Article> for SectionChild {
        fn from(value: crate::generated::all::Article) -> Self {
            Self::Article(value)
        }
    }
    impl std::convert::From<crate::generated::all::Aside> for SectionChild {
        fn from(value: crate::generated::all::Aside) -> Self {
            Self::Aside(value)
        }
    }
    impl std::convert::From<crate::generated::all::BlockQuote> for SectionChild {
        fn from(value: crate::generated::all::BlockQuote) -> Self {
            Self::BlockQuote(value)
        }
    }
    impl std::convert::From<crate::generated::all::DescriptionList> for SectionChild {
        fn from(value: crate::generated::all::DescriptionList) -> Self {
            Self::DescriptionList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Details> for SectionChild {
        fn from(value: crate::generated::all::Details) -> Self {
            Self::Details(value)
        }
    }
    impl std::convert::From<crate::generated::all::Dialog> for SectionChild {
        fn from(value: crate::generated::all::Dialog) -> Self {
            Self::Dialog(value)
        }
    }
    impl std::convert::From<crate::generated::all::Division> for SectionChild {
        fn from(value: crate::generated::all::Division) -> Self {
            Self::Division(value)
        }
    }
    impl std::convert::From<crate::generated::all::Fieldset> for SectionChild {
        fn from(value: crate::generated::all::Fieldset) -> Self {
            Self::Fieldset(value)
        }
    }
    impl std::convert::From<crate::generated::all::Figure> for SectionChild {
        fn from(value: crate::generated::all::Figure) -> Self {
            Self::Figure(value)
        }
    }
    impl std::convert::From<crate::generated::all::Footer> for SectionChild {
        fn from(value: crate::generated::all::Footer) -> Self {
            Self::Footer(value)
        }
    }
    impl std::convert::From<crate::generated::all::Form> for SectionChild {
        fn from(value: crate::generated::all::Form) -> Self {
            Self::Form(value)
        }
    }
    impl std::convert::From<crate::generated::all::Header> for SectionChild {
        fn from(value: crate::generated::all::Header) -> Self {
            Self::Header(value)
        }
    }
    impl std::convert::From<crate::generated::all::Main> for SectionChild {
        fn from(value: crate::generated::all::Main) -> Self {
            Self::Main(value)
        }
    }
    impl std::convert::From<crate::generated::all::Menu> for SectionChild {
        fn from(value: crate::generated::all::Menu) -> Self {
            Self::Menu(value)
        }
    }
    impl std::convert::From<crate::generated::all::Navigation> for SectionChild {
        fn from(value: crate::generated::all::Navigation) -> Self {
            Self::Navigation(value)
        }
    }
    impl std::convert::From<crate::generated::all::OrderedList> for SectionChild {
        fn from(value: crate::generated::all::OrderedList) -> Self {
            Self::OrderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Paragraph> for SectionChild {
        fn from(value: crate::generated::all::Paragraph) -> Self {
            Self::Paragraph(value)
        }
    }
    impl std::convert::From<crate::generated::all::PreformattedText> for SectionChild {
        fn from(value: crate::generated::all::PreformattedText) -> Self {
            Self::PreformattedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Section> for SectionChild {
        fn from(value: crate::generated::all::Section) -> Self {
            Self::Section(value)
        }
    }
    impl std::convert::From<crate::generated::all::Table> for SectionChild {
        fn from(value: crate::generated::all::Table) -> Self {
            Self::Table(value)
        }
    }
    impl std::convert::From<crate::generated::all::ThematicBreak> for SectionChild {
        fn from(value: crate::generated::all::ThematicBreak) -> Self {
            Self::ThematicBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::UnorderedList> for SectionChild {
        fn from(value: crate::generated::all::UnorderedList) -> Self {
            Self::UnorderedList(value)
        }
    }
    impl std::fmt::Display for SectionChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Address(el) => write!(f, "{el}"),
                Self::Article(el) => write!(f, "{el}"),
                Self::Aside(el) => write!(f, "{el}"),
                Self::BlockQuote(el) => write!(f, "{el}"),
                Self::DescriptionList(el) => write!(f, "{el}"),
                Self::Details(el) => write!(f, "{el}"),
                Self::Dialog(el) => write!(f, "{el}"),
                Self::Division(el) => write!(f, "{el}"),
                Self::Fieldset(el) => write!(f, "{el}"),
                Self::Figure(el) => write!(f, "{el}"),
                Self::Footer(el) => write!(f, "{el}"),
                Self::Form(el) => write!(f, "{el}"),
                Self::Header(el) => write!(f, "{el}"),
                Self::Main(el) => write!(f, "{el}"),
                Self::Menu(el) => write!(f, "{el}"),
                Self::Navigation(el) => write!(f, "{el}"),
                Self::OrderedList(el) => write!(f, "{el}"),
                Self::Paragraph(el) => write!(f, "{el}"),
                Self::PreformattedText(el) => write!(f, "{el}"),
                Self::Section(el) => write!(f, "{el}"),
                Self::Table(el) => write!(f, "{el}"),
                Self::ThematicBreak(el) => write!(f, "{el}"),
                Self::UnorderedList(el) => write!(f, "{el}"),
            }
        }
    }
}
pub mod builder {
    /// A builder struct for Section
    pub struct SectionBuilder {
        element: super::element::Section,
    }
    impl SectionBuilder {
        pub(crate) fn new(element: super::element::Section) -> Self {
            Self { element }
        }
        /// Finish building the element
        pub fn build(&mut self) -> super::element::Section {
            self.element.clone()
        }
        /// Append a new `Address` element
        pub fn address<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::AddressBuilder),
        {
            let ty: crate::generated::all::Address = Default::default();
            let mut ty_builder = crate::generated::all::builders::AddressBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Article` element
        pub fn article<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ArticleBuilder),
        {
            let ty: crate::generated::all::Article = Default::default();
            let mut ty_builder = crate::generated::all::builders::ArticleBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Aside` element
        pub fn aside<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::AsideBuilder),
        {
            let ty: crate::generated::all::Aside = Default::default();
            let mut ty_builder = crate::generated::all::builders::AsideBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `BlockQuote` element
        pub fn block_quote<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::BlockQuoteBuilder),
        {
            let ty: crate::generated::all::BlockQuote = Default::default();
            let mut ty_builder = crate::generated::all::builders::BlockQuoteBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `DescriptionList` element
        pub fn description_list<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DescriptionListBuilder),
        {
            let ty: crate::generated::all::DescriptionList = Default::default();
            let mut ty_builder = crate::generated::all::builders::DescriptionListBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Details` element
        pub fn details<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DetailsBuilder),
        {
            let ty: crate::generated::all::Details = Default::default();
            let mut ty_builder = crate::generated::all::builders::DetailsBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Dialog` element
        pub fn dialog<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DialogBuilder),
        {
            let ty: crate::generated::all::Dialog = Default::default();
            let mut ty_builder = crate::generated::all::builders::DialogBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Division` element
        pub fn division<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::DivisionBuilder),
        {
            let ty: crate::generated::all::Division = Default::default();
            let mut ty_builder = crate::generated::all::builders::DivisionBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Fieldset` element
        pub fn fieldset<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::FieldsetBuilder),
        {
            let ty: crate::generated::all::Fieldset = Default::default();
            let mut ty_builder = crate::generated::all::builders::FieldsetBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Figure` element
        pub fn figure<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::FigureBuilder),
        {
            let ty: crate::generated::all::Figure = Default::default();
            let mut ty_builder = crate::generated::all::builders::FigureBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Footer` element
        pub fn footer<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::FooterBuilder),
        {
            let ty: crate::generated::all::Footer = Default::default();
            let mut ty_builder = crate::generated::all::builders::FooterBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Form` element
        pub fn form<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::FormBuilder),
        {
            let ty: crate::generated::all::Form = Default::default();
            let mut ty_builder = crate::generated::all::builders::FormBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Header` element
        pub fn header<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::HeaderBuilder),
        {
            let ty: crate::generated::all::Header = Default::default();
            let mut ty_builder = crate::generated::all::builders::HeaderBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Main` element
        pub fn main<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::MainBuilder),
        {
            let ty: crate::generated::all::Main = Default::default();
            let mut ty_builder = crate::generated::all::builders::MainBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Menu` element
        pub fn menu<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::MenuBuilder),
        {
            let ty: crate::generated::all::Menu = Default::default();
            let mut ty_builder = crate::generated::all::builders::MenuBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Navigation` element
        pub fn navigation<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::NavigationBuilder),
        {
            let ty: crate::generated::all::Navigation = Default::default();
            let mut ty_builder = crate::generated::all::builders::NavigationBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `OrderedList` element
        pub fn ordered_list<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::OrderedListBuilder),
        {
            let ty: crate::generated::all::OrderedList = Default::default();
            let mut ty_builder = crate::generated::all::builders::OrderedListBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Paragraph` element
        pub fn paragraph<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ParagraphBuilder),
        {
            let ty: crate::generated::all::Paragraph = Default::default();
            let mut ty_builder = crate::generated::all::builders::ParagraphBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `PreformattedText` element
        pub fn preformatted_text<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::PreformattedTextBuilder),
        {
            let ty: crate::generated::all::PreformattedText = Default::default();
            let mut ty_builder = crate::generated::all::builders::PreformattedTextBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Section` element
        pub fn section<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::SectionBuilder),
        {
            let ty: crate::generated::all::Section = Default::default();
            let mut ty_builder = crate::generated::all::builders::SectionBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Table` element
        pub fn table<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::TableBuilder),
        {
            let ty: crate::generated::all::Table = Default::default();
            let mut ty_builder = crate::generated::all::builders::TableBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `ThematicBreak` element
        pub fn thematic_break<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::ThematicBreakBuilder),
        {
            let ty: crate::generated::all::ThematicBreak = Default::default();
            let mut ty_builder = crate::generated::all::builders::ThematicBreakBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `UnorderedList` element
        pub fn unordered_list<F>(&mut self, f: F) -> &mut Self
        where
            F: FnOnce(&mut crate::generated::all::builders::UnorderedListBuilder),
        {
            let ty: crate::generated::all::UnorderedList = Default::default();
            let mut ty_builder = crate::generated::all::builders::UnorderedListBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
    }
}
