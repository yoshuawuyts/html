pub mod element {
    /// The HTML `<meta>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
    #[doc(alias = "meta")]
    #[non_exhaustive]
    #[derive(PartialEq, Clone, Default)]
    pub struct Meta {
        sys: html_sys::metadata::Meta,
    }
    impl Meta {
        /// Create a new builder
        pub fn builder() -> super::builder::MetaBuilder {
            super::builder::MetaBuilder::new(Default::default())
        }
    }
    impl Meta {
        /// Access the element's `data-*` properties
        pub fn data_map(&self) -> &html_sys::DataMap {
            &self.sys.data_map
        }
        /// Mutably access the element's `data-*` properties
        pub fn data_map_mut(&mut self) -> &mut html_sys::DataMap {
            &mut self.sys.data_map
        }
    }
    impl Meta {
        /// Access the element's class set
        pub fn class_set(&self) -> &html_sys::ClassSet {
            &std::ops::Deref::deref(&self.sys).class_set
        }
        /// Mutably access the element's class set
        pub fn class_set_mut(&mut self) -> &mut html_sys::ClassSet {
            &mut std::ops::DerefMut::deref_mut(&mut self.sys).class_set
        }
    }
    impl Meta {
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
        /// Get the value of the `http-equiv` attribute
        pub fn http_equiv(&self) -> std::option::Option<&str> {
            self.sys.http_equiv.as_deref()
        }
        /// Set the value of the `http-equiv` attribute
        pub fn set_http_equiv(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.http_equiv = value.map(|v| v.into());
        }
        /// Get the value of the `content` attribute
        pub fn content(&self) -> std::option::Option<&str> {
            self.sys.content.as_deref()
        }
        /// Set the value of the `content` attribute
        pub fn set_content(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.content = value.map(|v| v.into());
        }
        /// Get the value of the `charset` attribute
        pub fn charset(&self) -> std::option::Option<&str> {
            self.sys.charset.as_deref()
        }
        /// Set the value of the `charset` attribute
        pub fn set_charset(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.charset = value.map(|v| v.into());
        }
        /// Get the value of the `media` attribute
        pub fn media(&self) -> std::option::Option<&str> {
            self.sys.media.as_deref()
        }
        /// Set the value of the `media` attribute
        pub fn set_media(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.media = value.map(|v| v.into());
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
    impl crate::Render for Meta {
        fn render(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            write!(f, "{:level$}", "", level = depth * 4)?;
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl std::fmt::Debug for Meta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            crate::Render::render(self, f, 0)?;
            Ok(())
        }
    }
    impl std::fmt::Display for Meta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for Meta {}
    impl crate::MetadataContent for Meta {}
    impl crate::FlowContent for Meta {}
    impl crate::PhrasingContent for Meta {}
    impl std::convert::Into<html_sys::metadata::Meta> for Meta {
        fn into(self) -> html_sys::metadata::Meta {
            self.sys
        }
    }
    impl From<html_sys::metadata::Meta> for Meta {
        fn from(sys: html_sys::metadata::Meta) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
pub mod builder {
    /// A builder struct for Meta
    pub struct MetaBuilder {
        element: super::element::Meta,
    }
    impl MetaBuilder {
        pub(crate) fn new(element: super::element::Meta) -> Self {
            Self { element }
        }
        /// Finish building the element
        pub fn build(&mut self) -> super::element::Meta {
            self.element.clone()
        }
        /// Insert a `data-*` property
        pub fn data(
            &mut self,
            data_key: impl Into<std::borrow::Cow<'static, str>>,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut MetaBuilder {
            self.element.data_map_mut().insert(data_key.into(), value.into());
            self
        }
        /// Insert a class name
        pub fn class(
            &mut self,
            class_name: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut MetaBuilder {
            self.element.class_set_mut().insert(class_name.into());
            self
        }
        /// Set the value of the `name` attribute
        pub fn name(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_name(Some(value.into()));
            self
        }
        /// Set the value of the `http-equiv` attribute
        pub fn http_equiv(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_http_equiv(Some(value.into()));
            self
        }
        /// Set the value of the `content` attribute
        pub fn content(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_content(Some(value.into()));
            self
        }
        /// Set the value of the `charset` attribute
        pub fn charset(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_charset(Some(value.into()));
            self
        }
        /// Set the value of the `media` attribute
        pub fn media(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_media(Some(value.into()));
            self
        }
        /// Set the value of the `accesskey` attribute
        pub fn access_key(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_access_key(Some(value.into()));
            self
        }
        /// Set the value of the `autocapitalize` attribute
        pub fn auto_capitalize(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_auto_capitalize(Some(value.into()));
            self
        }
        /// Set the value of the `autofocus` attribute
        pub fn autofocus(&mut self, value: bool) -> &mut Self {
            self.element.set_autofocus(value);
            self
        }
        /// Set the value of the `contenteditable` attribute
        pub fn content_editable(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_content_editable(Some(value.into()));
            self
        }
        /// Set the value of the `dir` attribute
        pub fn direction(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_direction(Some(value.into()));
            self
        }
        /// Set the value of the `draggable` attribute
        pub fn draggable(&mut self, value: bool) -> &mut Self {
            self.element.set_draggable(value);
            self
        }
        /// Set the value of the `enterkeyhint` attribute
        pub fn enter_key_hint(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_enter_key_hint(Some(value.into()));
            self
        }
        /// Set the value of the `exportparts` attribute
        pub fn export_parts(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_export_parts(Some(value.into()));
            self
        }
        /// Set the value of the `hidden` attribute
        pub fn hidden(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_hidden(Some(value.into()));
            self
        }
        /// Set the value of the `id` attribute
        pub fn id(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_id(Some(value.into()));
            self
        }
        /// Set the value of the `inert` attribute
        pub fn inert(&mut self, value: bool) -> &mut Self {
            self.element.set_inert(value);
            self
        }
        /// Set the value of the `inputmode` attribute
        pub fn input_mode(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_input_mode(Some(value.into()));
            self
        }
        /// Set the value of the `is` attribute
        pub fn is_(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_is_(Some(value.into()));
            self
        }
        /// Set the value of the `itemid` attribute
        pub fn item_id(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_item_id(Some(value.into()));
            self
        }
        /// Set the value of the `itemprop` attribute
        pub fn item_prop(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_item_prop(Some(value.into()));
            self
        }
        /// Set the value of the `itemref` attribute
        pub fn item_ref(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_item_ref(Some(value.into()));
            self
        }
        /// Set the value of the `itemscope` attribute
        pub fn item_scope(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_item_scope(Some(value.into()));
            self
        }
        /// Set the value of the `itemtype` attribute
        pub fn item_type(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_item_type(Some(value.into()));
            self
        }
        /// Set the value of the `lang` attribute
        pub fn lang(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_lang(Some(value.into()));
            self
        }
        /// Set the value of the `nonce` attribute
        pub fn nonce(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_nonce(Some(value.into()));
            self
        }
        /// Set the value of the `part` attribute
        pub fn part(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_part(Some(value.into()));
            self
        }
        /// Set the value of the `slot` attribute
        pub fn slot(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_slot(Some(value.into()));
            self
        }
        /// Set the value of the `spellcheck` attribute
        pub fn spellcheck(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_spellcheck(Some(value.into()));
            self
        }
        /// Set the value of the `style` attribute
        pub fn style(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_style(Some(value.into()));
            self
        }
        /// Set the value of the `tabindex` attribute
        pub fn tab_index(&mut self, value: i64) -> &mut Self {
            self.element.set_tab_index(Some(value));
            self
        }
        /// Set the value of the `title` attribute
        pub fn title(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_title(Some(value.into()));
            self
        }
        /// Set the value of the `translate` attribute
        pub fn translate(&mut self, value: bool) -> &mut Self {
            self.element.set_translate(value);
            self
        }
    }
}
