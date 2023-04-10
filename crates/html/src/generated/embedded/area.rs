pub mod element {
    /// The HTML `<area>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
    #[doc(alias = "area")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct ImageMapArea {
        sys: html_sys::embedded::ImageMapArea,
    }
    impl ImageMapArea {
        /// Get the value of the `alt` attribute
        pub fn alt(&self) -> std::option::Option<&str> {
            self.sys.alt.as_deref()
        }
        /// Set the value of the `alt` attribute
        pub fn set_alt(&mut self, value: std::option::Option<String>) {
            self.sys.alt = value;
        }
        /// Get the value of the `coords` attribute
        pub fn coords(&self) -> std::option::Option<&str> {
            self.sys.coords.as_deref()
        }
        /// Set the value of the `coords` attribute
        pub fn set_coords(&mut self, value: std::option::Option<String>) {
            self.sys.coords = value;
        }
        /// Get the value of the `shape` attribute
        pub fn shape(&self) -> std::option::Option<&str> {
            self.sys.shape.as_deref()
        }
        /// Set the value of the `shape` attribute
        pub fn set_shape(&mut self, value: std::option::Option<String>) {
            self.sys.shape = value;
        }
        /// Get the value of the `href` attribute
        pub fn href(&self) -> std::option::Option<&str> {
            self.sys.href.as_deref()
        }
        /// Set the value of the `href` attribute
        pub fn set_href(&mut self, value: std::option::Option<String>) {
            self.sys.href = value;
        }
        /// Get the value of the `target` attribute
        pub fn target(&self) -> std::option::Option<&str> {
            self.sys.target.as_deref()
        }
        /// Set the value of the `target` attribute
        pub fn set_target(&mut self, value: std::option::Option<String>) {
            self.sys.target = value;
        }
        /// Get the value of the `download` attribute
        pub fn download(&self) -> std::option::Option<&str> {
            self.sys.download.as_deref()
        }
        /// Set the value of the `download` attribute
        pub fn set_download(&mut self, value: std::option::Option<String>) {
            self.sys.download = value;
        }
        /// Get the value of the `ping` attribute
        pub fn ping(&self) -> std::option::Option<&str> {
            self.sys.ping.as_deref()
        }
        /// Set the value of the `ping` attribute
        pub fn set_ping(&mut self, value: std::option::Option<String>) {
            self.sys.ping = value;
        }
        /// Get the value of the `rel` attribute
        pub fn rel(&self) -> std::option::Option<&str> {
            self.sys.rel.as_deref()
        }
        /// Set the value of the `rel` attribute
        pub fn set_rel(&mut self, value: std::option::Option<String>) {
            self.sys.rel = value;
        }
        /// Get the value of the `referrerpolicy` attribute
        pub fn referrerpolicy(&self) -> std::option::Option<&str> {
            self.sys.referrerpolicy.as_deref()
        }
        /// Set the value of the `referrerpolicy` attribute
        pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
            self.sys.referrerpolicy = value;
        }
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
    impl std::fmt::Display for ImageMapArea {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for ImageMapArea {}
    impl crate::FlowContent for ImageMapArea {}
    impl crate::PhrasingContent for ImageMapArea {}
    impl std::convert::Into<html_sys::embedded::ImageMapArea> for ImageMapArea {
        fn into(self) -> html_sys::embedded::ImageMapArea {
            self.sys
        }
    }
    impl From<html_sys::embedded::ImageMapArea> for ImageMapArea {
        fn from(sys: html_sys::embedded::ImageMapArea) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
