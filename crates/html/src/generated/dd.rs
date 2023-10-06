pub mod element {
    /// The HTML `<dd>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
    #[doc(alias = "dd")]
    #[non_exhaustive]
    #[derive(PartialEq, Clone, Default)]
    pub struct DescriptionDetails {
        sys: html_sys::text::DescriptionDetails,
        children: Vec<super::child::DescriptionDetailsChild>,
    }
    impl DescriptionDetails {
        /// Create a new builder
        pub fn builder() -> super::builder::DescriptionDetailsBuilder {
            super::builder::DescriptionDetailsBuilder::new(Default::default())
        }
    }
    impl DescriptionDetails {
        /// Access the element's `data-*` properties
        pub fn data_map(&self) -> &html_sys::DataMap {
            &self.sys.data_map
        }
        /// Mutably access the element's `data-*` properties
        pub fn data_map_mut(&mut self) -> &mut html_sys::DataMap {
            &mut self.sys.data_map
        }
    }
    impl DescriptionDetails {
        /// Get the value of the `role` attribute
        pub fn role(&self) -> std::option::Option<&str> {
            self.sys.role.as_deref()
        }
        /// Set the value of the `role` attribute
        pub fn set_role(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.role = value.map(|v| v.into());
        }
        /// Get the value of the `aria-atomic` attribute
        pub fn aria_atomic(&self) -> bool {
            self.sys.aria_atomic
        }
        /// Set the value of the `aria-atomic` attribute
        pub fn set_aria_atomic(&mut self, value: bool) {
            self.sys.aria_atomic = value;
        }
        /// Get the value of the `aria-braillelabel` attribute
        pub fn aria_braille_label(&self) -> std::option::Option<&str> {
            self.sys.aria_braille_label.as_deref()
        }
        /// Set the value of the `aria-braillelabel` attribute
        pub fn set_aria_braille_label(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_braille_label = value.map(|v| v.into());
        }
        /// Get the value of the `aria-brailleroledescription` attribute
        pub fn aria_braille_role_description(&self) -> std::option::Option<&str> {
            self.sys.aria_braille_role_description.as_deref()
        }
        /// Set the value of the `aria-brailleroledescription` attribute
        pub fn set_aria_braille_role_description(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_braille_role_description = value.map(|v| v.into());
        }
        /// Get the value of the `aria-busy` attribute
        pub fn aria_busy(&self) -> bool {
            self.sys.aria_busy
        }
        /// Set the value of the `aria-busy` attribute
        pub fn set_aria_busy(&mut self, value: bool) {
            self.sys.aria_busy = value;
        }
        /// Get the value of the `aria-controls` attribute
        pub fn aria_controls_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_controls_elements.as_deref()
        }
        /// Set the value of the `aria-controls` attribute
        pub fn set_aria_controls_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_controls_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-current` attribute
        pub fn aria_current(&self) -> std::option::Option<&str> {
            self.sys.aria_current.as_deref()
        }
        /// Set the value of the `aria-current` attribute
        pub fn set_aria_current(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_current = value.map(|v| v.into());
        }
        /// Get the value of the `aria-describedby` attribute
        pub fn aria_described_by_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_described_by_elements.as_deref()
        }
        /// Set the value of the `aria-describedby` attribute
        pub fn set_aria_described_by_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_described_by_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-description` attribute
        pub fn aria_description(&self) -> std::option::Option<&str> {
            self.sys.aria_description.as_deref()
        }
        /// Set the value of the `aria-description` attribute
        pub fn set_aria_description(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_description = value.map(|v| v.into());
        }
        /// Get the value of the `aria-details` attribute
        pub fn aria_details_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_details_elements.as_deref()
        }
        /// Set the value of the `aria-details` attribute
        pub fn set_aria_details_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_details_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-disabled` attribute
        pub fn aria_disabled(&self) -> bool {
            self.sys.aria_disabled
        }
        /// Set the value of the `aria-disabled` attribute
        pub fn set_aria_disabled(&mut self, value: bool) {
            self.sys.aria_disabled = value;
        }
        /// Get the value of the `aria-dropeffect` attribute
        pub fn aria_drop_effect(&self) -> std::option::Option<&str> {
            self.sys.aria_drop_effect.as_deref()
        }
        /// Set the value of the `aria-dropeffect` attribute
        pub fn set_aria_drop_effect(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_drop_effect = value.map(|v| v.into());
        }
        /// Get the value of the `aria-errormessage` attribute
        pub fn aria_error_message_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_error_message_elements.as_deref()
        }
        /// Set the value of the `aria-errormessage` attribute
        pub fn set_aria_error_message_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_error_message_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-flowto` attribute
        pub fn aria_flow_to_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_flow_to_elements.as_deref()
        }
        /// Set the value of the `aria-flowto` attribute
        pub fn set_aria_flow_to_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_flow_to_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-grabbed` attribute
        pub fn aria_grabbed(&self) -> bool {
            self.sys.aria_grabbed
        }
        /// Set the value of the `aria-grabbed` attribute
        pub fn set_aria_grabbed(&mut self, value: bool) {
            self.sys.aria_grabbed = value;
        }
        /// Get the value of the `aria-haspopup` attribute
        pub fn aria_has_popup(&self) -> std::option::Option<&str> {
            self.sys.aria_has_popup.as_deref()
        }
        /// Set the value of the `aria-haspopup` attribute
        pub fn set_aria_has_popup(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_has_popup = value.map(|v| v.into());
        }
        /// Get the value of the `aria-hidden` attribute
        pub fn aria_hidden(&self) -> bool {
            self.sys.aria_hidden
        }
        /// Set the value of the `aria-hidden` attribute
        pub fn set_aria_hidden(&mut self, value: bool) {
            self.sys.aria_hidden = value;
        }
        /// Get the value of the `aria-invalid` attribute
        pub fn aria_invalid(&self) -> std::option::Option<&str> {
            self.sys.aria_invalid.as_deref()
        }
        /// Set the value of the `aria-invalid` attribute
        pub fn set_aria_invalid(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_invalid = value.map(|v| v.into());
        }
        /// Get the value of the `aria-keyshortcuts` attribute
        pub fn aria_key_shortcuts(&self) -> std::option::Option<&str> {
            self.sys.aria_key_shortcuts.as_deref()
        }
        /// Set the value of the `aria-keyshortcuts` attribute
        pub fn set_aria_key_shortcuts(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_key_shortcuts = value.map(|v| v.into());
        }
        /// Get the value of the `aria-label` attribute
        pub fn aria_label(&self) -> std::option::Option<&str> {
            self.sys.aria_label.as_deref()
        }
        /// Set the value of the `aria-label` attribute
        pub fn set_aria_label(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_label = value.map(|v| v.into());
        }
        /// Get the value of the `aria-labelledby` attribute
        pub fn aria_labelled_by_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_labelled_by_elements.as_deref()
        }
        /// Set the value of the `aria-labelledby` attribute
        pub fn set_aria_labelled_by_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_labelled_by_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-live` attribute
        pub fn aria_live(&self) -> std::option::Option<&str> {
            self.sys.aria_live.as_deref()
        }
        /// Set the value of the `aria-live` attribute
        pub fn set_aria_live(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_live = value.map(|v| v.into());
        }
        /// Get the value of the `aria-owns` attribute
        pub fn aria_owns_elements(&self) -> std::option::Option<&str> {
            self.sys.aria_owns_elements.as_deref()
        }
        /// Set the value of the `aria-owns` attribute
        pub fn set_aria_owns_elements(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_owns_elements = value.map(|v| v.into());
        }
        /// Get the value of the `aria-relevant` attribute
        pub fn aria_relevant(&self) -> std::option::Option<&str> {
            self.sys.aria_relevant.as_deref()
        }
        /// Set the value of the `aria-relevant` attribute
        pub fn set_aria_relevant(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_relevant = value.map(|v| v.into());
        }
        /// Get the value of the `aria-roledescription` attribute
        pub fn aria_role_description(&self) -> std::option::Option<&str> {
            self.sys.aria_role_description.as_deref()
        }
        /// Set the value of the `aria-roledescription` attribute
        pub fn set_aria_role_description(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_role_description = value.map(|v| v.into());
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
        pub fn class(&self) -> std::option::Option<&str> {
            self.sys.class.as_deref()
        }
        /// Set the value of the `class` attribute
        pub fn set_class(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.class = value.map(|v| v.into());
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
    impl DescriptionDetails {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::DescriptionDetailsChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(
            &mut self,
        ) -> &mut Vec<super::child::DescriptionDetailsChild> {
            &mut self.children
        }
    }
    impl crate::Render for DescriptionDetails {
        fn render(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            write!(f, "{:level$}", "", level = depth * 4)?;
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            if !self.children.is_empty() {
                write!(f, "\n")?;
            }
            for el in &self.children {
                crate::Render::render(&el, f, depth)?;
                write!(f, "\n")?;
            }
            write!(f, "{:level$}", "", level = depth * 4)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl std::fmt::Debug for DescriptionDetails {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            crate::Render::render(self, f, 0)?;
            Ok(())
        }
    }
    impl std::fmt::Display for DescriptionDetails {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                write!(f, "{el}")?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for DescriptionDetails {}
    impl std::convert::Into<html_sys::text::DescriptionDetails> for DescriptionDetails {
        fn into(self) -> html_sys::text::DescriptionDetails {
            self.sys
        }
    }
    impl From<html_sys::text::DescriptionDetails> for DescriptionDetails {
        fn from(sys: html_sys::text::DescriptionDetails) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `DescriptionDetails` element
    #[derive(PartialEq, Clone)]
    pub enum DescriptionDetailsChild {
        /// The Abbreviation element
        Abbreviation(crate::generated::all::Abbreviation),
        /// The Address element
        Address(crate::generated::all::Address),
        /// The Anchor element
        Anchor(crate::generated::all::Anchor),
        /// The Article element
        Article(crate::generated::all::Article),
        /// The Aside element
        Aside(crate::generated::all::Aside),
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
        /// The Heading1 element
        Heading1(crate::generated::all::Heading1),
        /// The Heading2 element
        Heading2(crate::generated::all::Heading2),
        /// The Heading3 element
        Heading3(crate::generated::all::Heading3),
        /// The Heading4 element
        Heading4(crate::generated::all::Heading4),
        /// The Heading5 element
        Heading5(crate::generated::all::Heading5),
        /// The Heading6 element
        Heading6(crate::generated::all::Heading6),
        /// The HeadingGroup element
        HeadingGroup(crate::generated::all::HeadingGroup),
        /// The Iframe element
        Iframe(crate::generated::all::Iframe),
        /// The Image element
        Image(crate::generated::all::Image),
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
        /// The Link element
        Link(crate::generated::all::Link),
        /// The Main element
        Main(crate::generated::all::Main),
        /// The MarkText element
        MarkText(crate::generated::all::MarkText),
        /// The Menu element
        Menu(crate::generated::all::Menu),
        /// The Meta element
        Meta(crate::generated::all::Meta),
        /// The Meter element
        Meter(crate::generated::all::Meter),
        /// The Navigation element
        Navigation(crate::generated::all::Navigation),
        /// The NoScript element
        NoScript(crate::generated::all::NoScript),
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
        /// The Search element
        Search(crate::generated::all::Search),
        /// The Section element
        Section(crate::generated::all::Section),
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
        /// The Text element
        Text(std::borrow::Cow<'static, str>),
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
    impl std::convert::From<crate::generated::all::Abbreviation>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Abbreviation) -> Self {
            Self::Abbreviation(value)
        }
    }
    impl std::convert::From<crate::generated::all::Address> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Address) -> Self {
            Self::Address(value)
        }
    }
    impl std::convert::From<crate::generated::all::Anchor> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Anchor) -> Self {
            Self::Anchor(value)
        }
    }
    impl std::convert::From<crate::generated::all::Article> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Article) -> Self {
            Self::Article(value)
        }
    }
    impl std::convert::From<crate::generated::all::Aside> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Aside) -> Self {
            Self::Aside(value)
        }
    }
    impl std::convert::From<crate::generated::all::Audio> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Audio) -> Self {
            Self::Audio(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalIsolate>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::BidirectionalIsolate) -> Self {
            Self::BidirectionalIsolate(value)
        }
    }
    impl std::convert::From<crate::generated::all::BidirectionalTextOverride>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::BidirectionalTextOverride) -> Self {
            Self::BidirectionalTextOverride(value)
        }
    }
    impl std::convert::From<crate::generated::all::BlockQuote>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::BlockQuote) -> Self {
            Self::BlockQuote(value)
        }
    }
    impl std::convert::From<crate::generated::all::Bold> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Bold) -> Self {
            Self::Bold(value)
        }
    }
    impl std::convert::From<crate::generated::all::Button> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Button) -> Self {
            Self::Button(value)
        }
    }
    impl std::convert::From<crate::generated::all::Canvas> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Canvas) -> Self {
            Self::Canvas(value)
        }
    }
    impl std::convert::From<crate::generated::all::Cite> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Cite) -> Self {
            Self::Cite(value)
        }
    }
    impl std::convert::From<crate::generated::all::Code> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Code) -> Self {
            Self::Code(value)
        }
    }
    impl std::convert::From<crate::generated::all::Data> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Data) -> Self {
            Self::Data(value)
        }
    }
    impl std::convert::From<crate::generated::all::DataList>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::DataList) -> Self {
            Self::DataList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Definition>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Definition) -> Self {
            Self::Definition(value)
        }
    }
    impl std::convert::From<crate::generated::all::DeletedText>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::DeletedText) -> Self {
            Self::DeletedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::DescriptionList>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::DescriptionList) -> Self {
            Self::DescriptionList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Details> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Details) -> Self {
            Self::Details(value)
        }
    }
    impl std::convert::From<crate::generated::all::Dialog> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Dialog) -> Self {
            Self::Dialog(value)
        }
    }
    impl std::convert::From<crate::generated::all::Division>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Division) -> Self {
            Self::Division(value)
        }
    }
    impl std::convert::From<crate::generated::all::Embed> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Embed) -> Self {
            Self::Embed(value)
        }
    }
    impl std::convert::From<crate::generated::all::Emphasis>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Emphasis) -> Self {
            Self::Emphasis(value)
        }
    }
    impl std::convert::From<crate::generated::all::Fieldset>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Fieldset) -> Self {
            Self::Fieldset(value)
        }
    }
    impl std::convert::From<crate::generated::all::Figure> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Figure) -> Self {
            Self::Figure(value)
        }
    }
    impl std::convert::From<crate::generated::all::Footer> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Footer) -> Self {
            Self::Footer(value)
        }
    }
    impl std::convert::From<crate::generated::all::Form> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Form) -> Self {
            Self::Form(value)
        }
    }
    impl std::convert::From<crate::generated::all::Header> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Header) -> Self {
            Self::Header(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading1>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Heading1) -> Self {
            Self::Heading1(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading2>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Heading2) -> Self {
            Self::Heading2(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading3>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Heading3) -> Self {
            Self::Heading3(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading4>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Heading4) -> Self {
            Self::Heading4(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading5>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Heading5) -> Self {
            Self::Heading5(value)
        }
    }
    impl std::convert::From<crate::generated::all::Heading6>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Heading6) -> Self {
            Self::Heading6(value)
        }
    }
    impl std::convert::From<crate::generated::all::HeadingGroup>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::HeadingGroup) -> Self {
            Self::HeadingGroup(value)
        }
    }
    impl std::convert::From<crate::generated::all::Iframe> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Iframe) -> Self {
            Self::Iframe(value)
        }
    }
    impl std::convert::From<crate::generated::all::Image> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Image) -> Self {
            Self::Image(value)
        }
    }
    impl std::convert::From<crate::generated::all::ImageMap>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::ImageMap) -> Self {
            Self::ImageMap(value)
        }
    }
    impl std::convert::From<crate::generated::all::ImageMapArea>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::ImageMapArea) -> Self {
            Self::ImageMapArea(value)
        }
    }
    impl std::convert::From<crate::generated::all::Input> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Input) -> Self {
            Self::Input(value)
        }
    }
    impl std::convert::From<crate::generated::all::InsertedText>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::InsertedText) -> Self {
            Self::InsertedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Italic> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Italic) -> Self {
            Self::Italic(value)
        }
    }
    impl std::convert::From<crate::generated::all::KeyboardInput>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::KeyboardInput) -> Self {
            Self::KeyboardInput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Label> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Label) -> Self {
            Self::Label(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreak>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::LineBreak) -> Self {
            Self::LineBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::LineBreakOpportunity>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::LineBreakOpportunity) -> Self {
            Self::LineBreakOpportunity(value)
        }
    }
    impl std::convert::From<crate::generated::all::Link> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Link) -> Self {
            Self::Link(value)
        }
    }
    impl std::convert::From<crate::generated::all::Main> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Main) -> Self {
            Self::Main(value)
        }
    }
    impl std::convert::From<crate::generated::all::MarkText>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::MarkText) -> Self {
            Self::MarkText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Menu> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Menu) -> Self {
            Self::Menu(value)
        }
    }
    impl std::convert::From<crate::generated::all::Meta> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Meta) -> Self {
            Self::Meta(value)
        }
    }
    impl std::convert::From<crate::generated::all::Meter> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Meter) -> Self {
            Self::Meter(value)
        }
    }
    impl std::convert::From<crate::generated::all::Navigation>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Navigation) -> Self {
            Self::Navigation(value)
        }
    }
    impl std::convert::From<crate::generated::all::NoScript>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::NoScript) -> Self {
            Self::NoScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::Object> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Object) -> Self {
            Self::Object(value)
        }
    }
    impl std::convert::From<crate::generated::all::OrderedList>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::OrderedList) -> Self {
            Self::OrderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Output> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Output) -> Self {
            Self::Output(value)
        }
    }
    impl std::convert::From<crate::generated::all::Paragraph>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Paragraph) -> Self {
            Self::Paragraph(value)
        }
    }
    impl std::convert::From<crate::generated::all::Picture> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Picture) -> Self {
            Self::Picture(value)
        }
    }
    impl std::convert::From<crate::generated::all::PreformattedText>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::PreformattedText) -> Self {
            Self::PreformattedText(value)
        }
    }
    impl std::convert::From<crate::generated::all::Progress>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Progress) -> Self {
            Self::Progress(value)
        }
    }
    impl std::convert::From<crate::generated::all::Quotation>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Quotation) -> Self {
            Self::Quotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::RubyAnnotation>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::RubyAnnotation) -> Self {
            Self::RubyAnnotation(value)
        }
    }
    impl std::convert::From<crate::generated::all::SampleOutput>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::SampleOutput) -> Self {
            Self::SampleOutput(value)
        }
    }
    impl std::convert::From<crate::generated::all::Script> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::Search> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Search) -> Self {
            Self::Search(value)
        }
    }
    impl std::convert::From<crate::generated::all::Section> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Section) -> Self {
            Self::Section(value)
        }
    }
    impl std::convert::From<crate::generated::all::Select> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Select) -> Self {
            Self::Select(value)
        }
    }
    impl std::convert::From<crate::generated::all::SideComment>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::SideComment) -> Self {
            Self::SideComment(value)
        }
    }
    impl std::convert::From<crate::generated::all::Slot> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Slot) -> Self {
            Self::Slot(value)
        }
    }
    impl std::convert::From<crate::generated::all::Span> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Span) -> Self {
            Self::Span(value)
        }
    }
    impl std::convert::From<crate::generated::all::StrikeThrough>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::StrikeThrough) -> Self {
            Self::StrikeThrough(value)
        }
    }
    impl std::convert::From<crate::generated::all::Strong> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Strong) -> Self {
            Self::Strong(value)
        }
    }
    impl std::convert::From<crate::generated::all::SubScript>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::SubScript) -> Self {
            Self::SubScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::SuperScript>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::SuperScript) -> Self {
            Self::SuperScript(value)
        }
    }
    impl std::convert::From<crate::generated::all::Table> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Table) -> Self {
            Self::Table(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl std::convert::From<std::borrow::Cow<'static, str>> for DescriptionDetailsChild {
        fn from(value: std::borrow::Cow<'static, str>) -> Self {
            Self::Text(value)
        }
    }
    impl std::convert::From<&'static str> for DescriptionDetailsChild {
        fn from(value: &'static str) -> Self {
            Self::Text(value.into())
        }
    }
    impl std::convert::From<String> for DescriptionDetailsChild {
        fn from(value: String) -> Self {
            Self::Text(value.into())
        }
    }
    impl std::convert::From<crate::generated::all::TextArea>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::TextArea) -> Self {
            Self::TextArea(value)
        }
    }
    impl std::convert::From<crate::generated::all::ThematicBreak>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::ThematicBreak) -> Self {
            Self::ThematicBreak(value)
        }
    }
    impl std::convert::From<crate::generated::all::Time> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Time) -> Self {
            Self::Time(value)
        }
    }
    impl std::convert::From<crate::generated::all::Underline>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Underline) -> Self {
            Self::Underline(value)
        }
    }
    impl std::convert::From<crate::generated::all::UnorderedList>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::UnorderedList) -> Self {
            Self::UnorderedList(value)
        }
    }
    impl std::convert::From<crate::generated::all::Variable>
    for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Variable) -> Self {
            Self::Variable(value)
        }
    }
    impl std::convert::From<crate::generated::all::Video> for DescriptionDetailsChild {
        fn from(value: crate::generated::all::Video) -> Self {
            Self::Video(value)
        }
    }
    impl crate::Render for DescriptionDetailsChild {
        fn render(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            match self {
                Self::Abbreviation(el) => crate::Render::render(el, f, depth + 1),
                Self::Address(el) => crate::Render::render(el, f, depth + 1),
                Self::Anchor(el) => crate::Render::render(el, f, depth + 1),
                Self::Article(el) => crate::Render::render(el, f, depth + 1),
                Self::Aside(el) => crate::Render::render(el, f, depth + 1),
                Self::Audio(el) => crate::Render::render(el, f, depth + 1),
                Self::BidirectionalIsolate(el) => crate::Render::render(el, f, depth + 1),
                Self::BidirectionalTextOverride(el) => {
                    crate::Render::render(el, f, depth + 1)
                }
                Self::BlockQuote(el) => crate::Render::render(el, f, depth + 1),
                Self::Bold(el) => crate::Render::render(el, f, depth + 1),
                Self::Button(el) => crate::Render::render(el, f, depth + 1),
                Self::Canvas(el) => crate::Render::render(el, f, depth + 1),
                Self::Cite(el) => crate::Render::render(el, f, depth + 1),
                Self::Code(el) => crate::Render::render(el, f, depth + 1),
                Self::Data(el) => crate::Render::render(el, f, depth + 1),
                Self::DataList(el) => crate::Render::render(el, f, depth + 1),
                Self::Definition(el) => crate::Render::render(el, f, depth + 1),
                Self::DeletedText(el) => crate::Render::render(el, f, depth + 1),
                Self::DescriptionList(el) => crate::Render::render(el, f, depth + 1),
                Self::Details(el) => crate::Render::render(el, f, depth + 1),
                Self::Dialog(el) => crate::Render::render(el, f, depth + 1),
                Self::Division(el) => crate::Render::render(el, f, depth + 1),
                Self::Embed(el) => crate::Render::render(el, f, depth + 1),
                Self::Emphasis(el) => crate::Render::render(el, f, depth + 1),
                Self::Fieldset(el) => crate::Render::render(el, f, depth + 1),
                Self::Figure(el) => crate::Render::render(el, f, depth + 1),
                Self::Footer(el) => crate::Render::render(el, f, depth + 1),
                Self::Form(el) => crate::Render::render(el, f, depth + 1),
                Self::Header(el) => crate::Render::render(el, f, depth + 1),
                Self::Heading1(el) => crate::Render::render(el, f, depth + 1),
                Self::Heading2(el) => crate::Render::render(el, f, depth + 1),
                Self::Heading3(el) => crate::Render::render(el, f, depth + 1),
                Self::Heading4(el) => crate::Render::render(el, f, depth + 1),
                Self::Heading5(el) => crate::Render::render(el, f, depth + 1),
                Self::Heading6(el) => crate::Render::render(el, f, depth + 1),
                Self::HeadingGroup(el) => crate::Render::render(el, f, depth + 1),
                Self::Iframe(el) => crate::Render::render(el, f, depth + 1),
                Self::Image(el) => crate::Render::render(el, f, depth + 1),
                Self::ImageMap(el) => crate::Render::render(el, f, depth + 1),
                Self::ImageMapArea(el) => crate::Render::render(el, f, depth + 1),
                Self::Input(el) => crate::Render::render(el, f, depth + 1),
                Self::InsertedText(el) => crate::Render::render(el, f, depth + 1),
                Self::Italic(el) => crate::Render::render(el, f, depth + 1),
                Self::KeyboardInput(el) => crate::Render::render(el, f, depth + 1),
                Self::Label(el) => crate::Render::render(el, f, depth + 1),
                Self::LineBreak(el) => crate::Render::render(el, f, depth + 1),
                Self::LineBreakOpportunity(el) => crate::Render::render(el, f, depth + 1),
                Self::Link(el) => crate::Render::render(el, f, depth + 1),
                Self::Main(el) => crate::Render::render(el, f, depth + 1),
                Self::MarkText(el) => crate::Render::render(el, f, depth + 1),
                Self::Menu(el) => crate::Render::render(el, f, depth + 1),
                Self::Meta(el) => crate::Render::render(el, f, depth + 1),
                Self::Meter(el) => crate::Render::render(el, f, depth + 1),
                Self::Navigation(el) => crate::Render::render(el, f, depth + 1),
                Self::NoScript(el) => crate::Render::render(el, f, depth + 1),
                Self::Object(el) => crate::Render::render(el, f, depth + 1),
                Self::OrderedList(el) => crate::Render::render(el, f, depth + 1),
                Self::Output(el) => crate::Render::render(el, f, depth + 1),
                Self::Paragraph(el) => crate::Render::render(el, f, depth + 1),
                Self::Picture(el) => crate::Render::render(el, f, depth + 1),
                Self::PreformattedText(el) => crate::Render::render(el, f, depth + 1),
                Self::Progress(el) => crate::Render::render(el, f, depth + 1),
                Self::Quotation(el) => crate::Render::render(el, f, depth + 1),
                Self::RubyAnnotation(el) => crate::Render::render(el, f, depth + 1),
                Self::SampleOutput(el) => crate::Render::render(el, f, depth + 1),
                Self::Script(el) => crate::Render::render(el, f, depth + 1),
                Self::Search(el) => crate::Render::render(el, f, depth + 1),
                Self::Section(el) => crate::Render::render(el, f, depth + 1),
                Self::Select(el) => crate::Render::render(el, f, depth + 1),
                Self::SideComment(el) => crate::Render::render(el, f, depth + 1),
                Self::Slot(el) => crate::Render::render(el, f, depth + 1),
                Self::Span(el) => crate::Render::render(el, f, depth + 1),
                Self::StrikeThrough(el) => crate::Render::render(el, f, depth + 1),
                Self::Strong(el) => crate::Render::render(el, f, depth + 1),
                Self::SubScript(el) => crate::Render::render(el, f, depth + 1),
                Self::SuperScript(el) => crate::Render::render(el, f, depth + 1),
                Self::Table(el) => crate::Render::render(el, f, depth + 1),
                Self::Template(el) => crate::Render::render(el, f, depth + 1),
                Self::Text(el) => crate::Render::render(el, f, depth + 1),
                Self::TextArea(el) => crate::Render::render(el, f, depth + 1),
                Self::ThematicBreak(el) => crate::Render::render(el, f, depth + 1),
                Self::Time(el) => crate::Render::render(el, f, depth + 1),
                Self::Underline(el) => crate::Render::render(el, f, depth + 1),
                Self::UnorderedList(el) => crate::Render::render(el, f, depth + 1),
                Self::Variable(el) => crate::Render::render(el, f, depth + 1),
                Self::Video(el) => crate::Render::render(el, f, depth + 1),
            }
        }
    }
    impl std::fmt::Debug for DescriptionDetailsChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            crate::Render::render(self, f, 0)?;
            Ok(())
        }
    }
    impl std::fmt::Display for DescriptionDetailsChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Abbreviation(el) => write!(f, "{el}"),
                Self::Address(el) => write!(f, "{el}"),
                Self::Anchor(el) => write!(f, "{el}"),
                Self::Article(el) => write!(f, "{el}"),
                Self::Aside(el) => write!(f, "{el}"),
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
                Self::Heading1(el) => write!(f, "{el}"),
                Self::Heading2(el) => write!(f, "{el}"),
                Self::Heading3(el) => write!(f, "{el}"),
                Self::Heading4(el) => write!(f, "{el}"),
                Self::Heading5(el) => write!(f, "{el}"),
                Self::Heading6(el) => write!(f, "{el}"),
                Self::HeadingGroup(el) => write!(f, "{el}"),
                Self::Iframe(el) => write!(f, "{el}"),
                Self::Image(el) => write!(f, "{el}"),
                Self::ImageMap(el) => write!(f, "{el}"),
                Self::ImageMapArea(el) => write!(f, "{el}"),
                Self::Input(el) => write!(f, "{el}"),
                Self::InsertedText(el) => write!(f, "{el}"),
                Self::Italic(el) => write!(f, "{el}"),
                Self::KeyboardInput(el) => write!(f, "{el}"),
                Self::Label(el) => write!(f, "{el}"),
                Self::LineBreak(el) => write!(f, "{el}"),
                Self::LineBreakOpportunity(el) => write!(f, "{el}"),
                Self::Link(el) => write!(f, "{el}"),
                Self::Main(el) => write!(f, "{el}"),
                Self::MarkText(el) => write!(f, "{el}"),
                Self::Menu(el) => write!(f, "{el}"),
                Self::Meta(el) => write!(f, "{el}"),
                Self::Meter(el) => write!(f, "{el}"),
                Self::Navigation(el) => write!(f, "{el}"),
                Self::NoScript(el) => write!(f, "{el}"),
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
                Self::Search(el) => write!(f, "{el}"),
                Self::Section(el) => write!(f, "{el}"),
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
                Self::Text(el) => write!(f, "{el}"),
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
pub mod builder {
    /// A builder struct for DescriptionDetails
    pub struct DescriptionDetailsBuilder {
        element: super::element::DescriptionDetails,
    }
    impl DescriptionDetailsBuilder {
        pub(crate) fn new(element: super::element::DescriptionDetails) -> Self {
            Self { element }
        }
        /// Finish building the element
        pub fn build(&mut self) -> super::element::DescriptionDetails {
            self.element.clone()
        }
        /// Insert a `data-*` property
        pub fn data(
            &mut self,
            data_key: impl Into<std::borrow::Cow<'static, str>>,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut DescriptionDetailsBuilder {
            self.element.data_map_mut().insert(data_key.into(), value.into());
            self
        }
        /// Append a new `Abbreviation` element
        pub fn abbreviation<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::AbbreviationBuilder,
            ) -> &'a mut crate::generated::all::builders::AbbreviationBuilder,
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
        /// Append a new `Address` element
        pub fn address<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::AddressBuilder,
            ) -> &'a mut crate::generated::all::builders::AddressBuilder,
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
        /// Append a new `Anchor` element
        pub fn anchor<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::AnchorBuilder,
            ) -> &'a mut crate::generated::all::builders::AnchorBuilder,
        {
            let ty: crate::generated::all::Anchor = Default::default();
            let mut ty_builder = crate::generated::all::builders::AnchorBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Article` element
        pub fn article<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ArticleBuilder,
            ) -> &'a mut crate::generated::all::builders::ArticleBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::AsideBuilder,
            ) -> &'a mut crate::generated::all::builders::AsideBuilder,
        {
            let ty: crate::generated::all::Aside = Default::default();
            let mut ty_builder = crate::generated::all::builders::AsideBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Audio` element
        pub fn audio<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::AudioBuilder,
            ) -> &'a mut crate::generated::all::builders::AudioBuilder,
        {
            let ty: crate::generated::all::Audio = Default::default();
            let mut ty_builder = crate::generated::all::builders::AudioBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `BidirectionalIsolate` element
        pub fn bidirectional_isolate<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::BidirectionalIsolateBuilder,
            ) -> &'a mut crate::generated::all::builders::BidirectionalIsolateBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::BidirectionalTextOverrideBuilder,
            ) -> &'a mut crate::generated::all::builders::BidirectionalTextOverrideBuilder,
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
        /// Append a new `BlockQuote` element
        pub fn block_quote<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::BlockQuoteBuilder,
            ) -> &'a mut crate::generated::all::builders::BlockQuoteBuilder,
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
        /// Append a new `Bold` element
        pub fn bold<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::BoldBuilder,
            ) -> &'a mut crate::generated::all::builders::BoldBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ButtonBuilder,
            ) -> &'a mut crate::generated::all::builders::ButtonBuilder,
        {
            let ty: crate::generated::all::Button = Default::default();
            let mut ty_builder = crate::generated::all::builders::ButtonBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Canvas` element
        pub fn canvas<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::CanvasBuilder,
            ) -> &'a mut crate::generated::all::builders::CanvasBuilder,
        {
            let ty: crate::generated::all::Canvas = Default::default();
            let mut ty_builder = crate::generated::all::builders::CanvasBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Cite` element
        pub fn cite<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::CiteBuilder,
            ) -> &'a mut crate::generated::all::builders::CiteBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::CodeBuilder,
            ) -> &'a mut crate::generated::all::builders::CodeBuilder,
        {
            let ty: crate::generated::all::Code = Default::default();
            let mut ty_builder = crate::generated::all::builders::CodeBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Data` element
        pub fn data_el<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DataBuilder,
            ) -> &'a mut crate::generated::all::builders::DataBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DataListBuilder,
            ) -> &'a mut crate::generated::all::builders::DataListBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DefinitionBuilder,
            ) -> &'a mut crate::generated::all::builders::DefinitionBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DeletedTextBuilder,
            ) -> &'a mut crate::generated::all::builders::DeletedTextBuilder,
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
        /// Append a new `DescriptionList` element
        pub fn description_list<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DescriptionListBuilder,
            ) -> &'a mut crate::generated::all::builders::DescriptionListBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DetailsBuilder,
            ) -> &'a mut crate::generated::all::builders::DetailsBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DialogBuilder,
            ) -> &'a mut crate::generated::all::builders::DialogBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::DivisionBuilder,
            ) -> &'a mut crate::generated::all::builders::DivisionBuilder,
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
        /// Append a new `Embed` element
        pub fn embed<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::EmbedBuilder,
            ) -> &'a mut crate::generated::all::builders::EmbedBuilder,
        {
            let ty: crate::generated::all::Embed = Default::default();
            let mut ty_builder = crate::generated::all::builders::EmbedBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Emphasis` element
        pub fn emphasis<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::EmphasisBuilder,
            ) -> &'a mut crate::generated::all::builders::EmphasisBuilder,
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
        /// Append a new `Fieldset` element
        pub fn fieldset<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::FieldsetBuilder,
            ) -> &'a mut crate::generated::all::builders::FieldsetBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::FigureBuilder,
            ) -> &'a mut crate::generated::all::builders::FigureBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::FooterBuilder,
            ) -> &'a mut crate::generated::all::builders::FooterBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::FormBuilder,
            ) -> &'a mut crate::generated::all::builders::FormBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::HeaderBuilder,
            ) -> &'a mut crate::generated::all::builders::HeaderBuilder,
        {
            let ty: crate::generated::all::Header = Default::default();
            let mut ty_builder = crate::generated::all::builders::HeaderBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Heading1` element
        pub fn heading_1<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::Heading1Builder,
            ) -> &'a mut crate::generated::all::builders::Heading1Builder,
        {
            let ty: crate::generated::all::Heading1 = Default::default();
            let mut ty_builder = crate::generated::all::builders::Heading1Builder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Heading2` element
        pub fn heading_2<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::Heading2Builder,
            ) -> &'a mut crate::generated::all::builders::Heading2Builder,
        {
            let ty: crate::generated::all::Heading2 = Default::default();
            let mut ty_builder = crate::generated::all::builders::Heading2Builder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Heading3` element
        pub fn heading_3<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::Heading3Builder,
            ) -> &'a mut crate::generated::all::builders::Heading3Builder,
        {
            let ty: crate::generated::all::Heading3 = Default::default();
            let mut ty_builder = crate::generated::all::builders::Heading3Builder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Heading4` element
        pub fn heading_4<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::Heading4Builder,
            ) -> &'a mut crate::generated::all::builders::Heading4Builder,
        {
            let ty: crate::generated::all::Heading4 = Default::default();
            let mut ty_builder = crate::generated::all::builders::Heading4Builder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Heading5` element
        pub fn heading_5<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::Heading5Builder,
            ) -> &'a mut crate::generated::all::builders::Heading5Builder,
        {
            let ty: crate::generated::all::Heading5 = Default::default();
            let mut ty_builder = crate::generated::all::builders::Heading5Builder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Heading6` element
        pub fn heading_6<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::Heading6Builder,
            ) -> &'a mut crate::generated::all::builders::Heading6Builder,
        {
            let ty: crate::generated::all::Heading6 = Default::default();
            let mut ty_builder = crate::generated::all::builders::Heading6Builder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `HeadingGroup` element
        pub fn heading_group<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::HeadingGroupBuilder,
            ) -> &'a mut crate::generated::all::builders::HeadingGroupBuilder,
        {
            let ty: crate::generated::all::HeadingGroup = Default::default();
            let mut ty_builder = crate::generated::all::builders::HeadingGroupBuilder::new(
                ty,
            );
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Iframe` element
        pub fn iframe<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::IframeBuilder,
            ) -> &'a mut crate::generated::all::builders::IframeBuilder,
        {
            let ty: crate::generated::all::Iframe = Default::default();
            let mut ty_builder = crate::generated::all::builders::IframeBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Image` element
        pub fn image<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ImageBuilder,
            ) -> &'a mut crate::generated::all::builders::ImageBuilder,
        {
            let ty: crate::generated::all::Image = Default::default();
            let mut ty_builder = crate::generated::all::builders::ImageBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `ImageMap` element
        pub fn image_map<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ImageMapBuilder,
            ) -> &'a mut crate::generated::all::builders::ImageMapBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ImageMapAreaBuilder,
            ) -> &'a mut crate::generated::all::builders::ImageMapAreaBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::InputBuilder,
            ) -> &'a mut crate::generated::all::builders::InputBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::InsertedTextBuilder,
            ) -> &'a mut crate::generated::all::builders::InsertedTextBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ItalicBuilder,
            ) -> &'a mut crate::generated::all::builders::ItalicBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::KeyboardInputBuilder,
            ) -> &'a mut crate::generated::all::builders::KeyboardInputBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::LabelBuilder,
            ) -> &'a mut crate::generated::all::builders::LabelBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::LineBreakBuilder,
            ) -> &'a mut crate::generated::all::builders::LineBreakBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::LineBreakOpportunityBuilder,
            ) -> &'a mut crate::generated::all::builders::LineBreakOpportunityBuilder,
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
        /// Append a new `Link` element
        pub fn link<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::LinkBuilder,
            ) -> &'a mut crate::generated::all::builders::LinkBuilder,
        {
            let ty: crate::generated::all::Link = Default::default();
            let mut ty_builder = crate::generated::all::builders::LinkBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Main` element
        pub fn main<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::MainBuilder,
            ) -> &'a mut crate::generated::all::builders::MainBuilder,
        {
            let ty: crate::generated::all::Main = Default::default();
            let mut ty_builder = crate::generated::all::builders::MainBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `MarkText` element
        pub fn mark_text<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::MarkTextBuilder,
            ) -> &'a mut crate::generated::all::builders::MarkTextBuilder,
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
        /// Append a new `Menu` element
        pub fn menu<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::MenuBuilder,
            ) -> &'a mut crate::generated::all::builders::MenuBuilder,
        {
            let ty: crate::generated::all::Menu = Default::default();
            let mut ty_builder = crate::generated::all::builders::MenuBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Meta` element
        pub fn meta<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::MetaBuilder,
            ) -> &'a mut crate::generated::all::builders::MetaBuilder,
        {
            let ty: crate::generated::all::Meta = Default::default();
            let mut ty_builder = crate::generated::all::builders::MetaBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Meter` element
        pub fn meter<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::MeterBuilder,
            ) -> &'a mut crate::generated::all::builders::MeterBuilder,
        {
            let ty: crate::generated::all::Meter = Default::default();
            let mut ty_builder = crate::generated::all::builders::MeterBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Navigation` element
        pub fn navigation<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::NavigationBuilder,
            ) -> &'a mut crate::generated::all::builders::NavigationBuilder,
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
        /// Append a new `NoScript` element
        pub fn no_script<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::NoScriptBuilder,
            ) -> &'a mut crate::generated::all::builders::NoScriptBuilder,
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
        /// Append a new `Object` element
        pub fn object<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ObjectBuilder,
            ) -> &'a mut crate::generated::all::builders::ObjectBuilder,
        {
            let ty: crate::generated::all::Object = Default::default();
            let mut ty_builder = crate::generated::all::builders::ObjectBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `OrderedList` element
        pub fn ordered_list<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::OrderedListBuilder,
            ) -> &'a mut crate::generated::all::builders::OrderedListBuilder,
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
        /// Append a new `Output` element
        pub fn output<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::OutputBuilder,
            ) -> &'a mut crate::generated::all::builders::OutputBuilder,
        {
            let ty: crate::generated::all::Output = Default::default();
            let mut ty_builder = crate::generated::all::builders::OutputBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Paragraph` element
        pub fn paragraph<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ParagraphBuilder,
            ) -> &'a mut crate::generated::all::builders::ParagraphBuilder,
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
        /// Append a new `Picture` element
        pub fn picture<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::PictureBuilder,
            ) -> &'a mut crate::generated::all::builders::PictureBuilder,
        {
            let ty: crate::generated::all::Picture = Default::default();
            let mut ty_builder = crate::generated::all::builders::PictureBuilder::new(
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::PreformattedTextBuilder,
            ) -> &'a mut crate::generated::all::builders::PreformattedTextBuilder,
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
        /// Append a new `Progress` element
        pub fn progress<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ProgressBuilder,
            ) -> &'a mut crate::generated::all::builders::ProgressBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::QuotationBuilder,
            ) -> &'a mut crate::generated::all::builders::QuotationBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::RubyAnnotationBuilder,
            ) -> &'a mut crate::generated::all::builders::RubyAnnotationBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SampleOutputBuilder,
            ) -> &'a mut crate::generated::all::builders::SampleOutputBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ScriptBuilder,
            ) -> &'a mut crate::generated::all::builders::ScriptBuilder,
        {
            let ty: crate::generated::all::Script = Default::default();
            let mut ty_builder = crate::generated::all::builders::ScriptBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Search` element
        pub fn search<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SearchBuilder,
            ) -> &'a mut crate::generated::all::builders::SearchBuilder,
        {
            let ty: crate::generated::all::Search = Default::default();
            let mut ty_builder = crate::generated::all::builders::SearchBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Section` element
        pub fn section<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SectionBuilder,
            ) -> &'a mut crate::generated::all::builders::SectionBuilder,
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
        /// Append a new `Select` element
        pub fn select<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SelectBuilder,
            ) -> &'a mut crate::generated::all::builders::SelectBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SideCommentBuilder,
            ) -> &'a mut crate::generated::all::builders::SideCommentBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SlotBuilder,
            ) -> &'a mut crate::generated::all::builders::SlotBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SpanBuilder,
            ) -> &'a mut crate::generated::all::builders::SpanBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::StrikeThroughBuilder,
            ) -> &'a mut crate::generated::all::builders::StrikeThroughBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::StrongBuilder,
            ) -> &'a mut crate::generated::all::builders::StrongBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SubScriptBuilder,
            ) -> &'a mut crate::generated::all::builders::SubScriptBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::SuperScriptBuilder,
            ) -> &'a mut crate::generated::all::builders::SuperScriptBuilder,
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
        /// Append a new `Table` element
        pub fn table<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::TableBuilder,
            ) -> &'a mut crate::generated::all::builders::TableBuilder,
        {
            let ty: crate::generated::all::Table = Default::default();
            let mut ty_builder = crate::generated::all::builders::TableBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Append a new `Template` element
        pub fn template<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::TemplateBuilder,
            ) -> &'a mut crate::generated::all::builders::TemplateBuilder,
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
        /// Append a new text element.
        pub fn text(
            &mut self,
            s: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            let cow = s.into();
            self.element.children_mut().push(cow.into());
            self
        }
        /// Append a new `TextArea` element
        pub fn text_area<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::TextAreaBuilder,
            ) -> &'a mut crate::generated::all::builders::TextAreaBuilder,
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
        /// Append a new `ThematicBreak` element
        pub fn thematic_break<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::ThematicBreakBuilder,
            ) -> &'a mut crate::generated::all::builders::ThematicBreakBuilder,
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
        /// Append a new `Time` element
        pub fn time<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::TimeBuilder,
            ) -> &'a mut crate::generated::all::builders::TimeBuilder,
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
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::UnderlineBuilder,
            ) -> &'a mut crate::generated::all::builders::UnderlineBuilder,
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
        /// Append a new `UnorderedList` element
        pub fn unordered_list<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::UnorderedListBuilder,
            ) -> &'a mut crate::generated::all::builders::UnorderedListBuilder,
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
        /// Append a new `Variable` element
        pub fn variable<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::VariableBuilder,
            ) -> &'a mut crate::generated::all::builders::VariableBuilder,
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
        /// Append a new `Video` element
        pub fn video<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::VideoBuilder,
            ) -> &'a mut crate::generated::all::builders::VideoBuilder,
        {
            let ty: crate::generated::all::Video = Default::default();
            let mut ty_builder = crate::generated::all::builders::VideoBuilder::new(ty);
            (f)(&mut ty_builder);
            let ty = ty_builder.build();
            self.element.children_mut().push(ty.into());
            self
        }
        /// Set the value of the `role` attribute
        pub fn role(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_role(Some(value.into()));
            self
        }
        /// Set the value of the `aria-atomic` attribute
        pub fn aria_atomic(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_atomic(value);
            self
        }
        /// Set the value of the `aria-braillelabel` attribute
        pub fn aria_braille_label(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_braille_label(Some(value.into()));
            self
        }
        /// Set the value of the `aria-brailleroledescription` attribute
        pub fn aria_braille_role_description(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_braille_role_description(Some(value.into()));
            self
        }
        /// Set the value of the `aria-busy` attribute
        pub fn aria_busy(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_busy(value);
            self
        }
        /// Set the value of the `aria-controls` attribute
        pub fn aria_controls_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_controls_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-current` attribute
        pub fn aria_current(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_current(Some(value.into()));
            self
        }
        /// Set the value of the `aria-describedby` attribute
        pub fn aria_described_by_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_described_by_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-description` attribute
        pub fn aria_description(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_description(Some(value.into()));
            self
        }
        /// Set the value of the `aria-details` attribute
        pub fn aria_details_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_details_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-disabled` attribute
        pub fn aria_disabled(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_disabled(value);
            self
        }
        /// Set the value of the `aria-dropeffect` attribute
        pub fn aria_drop_effect(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_drop_effect(Some(value.into()));
            self
        }
        /// Set the value of the `aria-errormessage` attribute
        pub fn aria_error_message_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_error_message_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-flowto` attribute
        pub fn aria_flow_to_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_flow_to_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-grabbed` attribute
        pub fn aria_grabbed(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_grabbed(value);
            self
        }
        /// Set the value of the `aria-haspopup` attribute
        pub fn aria_has_popup(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_has_popup(Some(value.into()));
            self
        }
        /// Set the value of the `aria-hidden` attribute
        pub fn aria_hidden(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_hidden(value);
            self
        }
        /// Set the value of the `aria-invalid` attribute
        pub fn aria_invalid(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_invalid(Some(value.into()));
            self
        }
        /// Set the value of the `aria-keyshortcuts` attribute
        pub fn aria_key_shortcuts(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_key_shortcuts(Some(value.into()));
            self
        }
        /// Set the value of the `aria-label` attribute
        pub fn aria_label(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_label(Some(value.into()));
            self
        }
        /// Set the value of the `aria-labelledby` attribute
        pub fn aria_labelled_by_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_labelled_by_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-live` attribute
        pub fn aria_live(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_live(Some(value.into()));
            self
        }
        /// Set the value of the `aria-owns` attribute
        pub fn aria_owns_elements(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_owns_elements(Some(value.into()));
            self
        }
        /// Set the value of the `aria-relevant` attribute
        pub fn aria_relevant(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_relevant(Some(value.into()));
            self
        }
        /// Set the value of the `aria-roledescription` attribute
        pub fn aria_role_description(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_role_description(Some(value.into()));
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
        /// Set the value of the `class` attribute
        pub fn class(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_class(Some(value.into()));
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
        pub fn slot_attr(
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
        /// Push a new child element to the list of children.
        pub fn push<T>(&mut self, child_el: T) -> &mut Self
        where
            T: Into<crate::generated::all::children::DescriptionDetailsChild>,
        {
            let child_el = child_el.into();
            self.element.children_mut().push(child_el);
            self
        }
        /// Extend the list of children with an iterator of child elements.
        pub fn extend<I, T>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = T>,
            T: Into<crate::generated::all::children::DescriptionDetailsChild>,
        {
            let iter = iter.into_iter().map(|child_el| child_el.into());
            self.element.children_mut().extend(iter);
            self
        }
    }
}
