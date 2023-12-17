pub mod element {
    /// The HTML `<tfoot>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
    #[doc(alias = "tfoot")]
    #[non_exhaustive]
    #[derive(PartialEq, Clone, Default)]
    pub struct TableFoot {
        sys: html_sys::tables::TableFoot,
        children: Vec<super::child::TableFootChild>,
    }
    impl TableFoot {
        /// Create a new builder
        pub fn builder() -> super::builder::TableFootBuilder {
            super::builder::TableFootBuilder::new(Default::default())
        }
    }
    impl TableFoot {
        /// Access the element's `data-*` properties
        pub fn data_map(&self) -> &html_sys::DataMap {
            &self.sys.data_map
        }
        /// Mutably access the element's `data-*` properties
        pub fn data_map_mut(&mut self) -> &mut html_sys::DataMap {
            &mut self.sys.data_map
        }
    }
    impl TableFoot {
        /// Access the element's class set
        pub fn class_set(&self) -> &html_sys::ClassSet {
            &std::ops::Deref::deref(&self.sys).class_set
        }
        /// Mutably access the element's class set
        pub fn class_set_mut(&mut self) -> &mut html_sys::ClassSet {
            &mut std::ops::DerefMut::deref_mut(&mut self.sys).class_set
        }
    }
    impl TableFoot {
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
        /// Get the value of the `aria-activedescendant` attribute
        pub fn aria_active_descendant_element(&self) -> std::option::Option<&str> {
            self.sys.aria_active_descendant_element.as_deref()
        }
        /// Set the value of the `aria-activedescendant` attribute
        pub fn set_aria_active_descendant_element(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_active_descendant_element = value.map(|v| v.into());
        }
        /// Get the value of the `aria-atomic` attribute
        pub fn aria_atomic(&self) -> bool {
            self.sys.aria_atomic
        }
        /// Set the value of the `aria-atomic` attribute
        pub fn set_aria_atomic(&mut self, value: bool) {
            self.sys.aria_atomic = value;
        }
        /// Get the value of the `aria-autocomplete` attribute
        pub fn aria_auto_complete(&self) -> std::option::Option<&str> {
            self.sys.aria_auto_complete.as_deref()
        }
        /// Set the value of the `aria-autocomplete` attribute
        pub fn set_aria_auto_complete(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_auto_complete = value.map(|v| v.into());
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
        /// Get the value of the `aria-checked` attribute
        pub fn aria_checked(&self) -> std::option::Option<&str> {
            self.sys.aria_checked.as_deref()
        }
        /// Set the value of the `aria-checked` attribute
        pub fn set_aria_checked(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_checked = value.map(|v| v.into());
        }
        /// Get the value of the `aria-colcount` attribute
        pub fn aria_col_count(&self) -> std::option::Option<i64> {
            self.sys.aria_col_count
        }
        /// Set the value of the `aria-colcount` attribute
        pub fn set_aria_col_count(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_col_count = value;
        }
        /// Get the value of the `aria-colindex` attribute
        pub fn aria_col_index(&self) -> std::option::Option<i64> {
            self.sys.aria_col_index
        }
        /// Set the value of the `aria-colindex` attribute
        pub fn set_aria_col_index(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_col_index = value;
        }
        /// Get the value of the `aria-colindextext` attribute
        pub fn aria_col_index_text(&self) -> std::option::Option<&str> {
            self.sys.aria_col_index_text.as_deref()
        }
        /// Set the value of the `aria-colindextext` attribute
        pub fn set_aria_col_index_text(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_col_index_text = value.map(|v| v.into());
        }
        /// Get the value of the `aria-colspan` attribute
        pub fn aria_col_span(&self) -> std::option::Option<i64> {
            self.sys.aria_col_span
        }
        /// Set the value of the `aria-colspan` attribute
        pub fn set_aria_col_span(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_col_span = value;
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
        /// Get the value of the `aria-expanded` attribute
        pub fn aria_expanded(&self) -> bool {
            self.sys.aria_expanded
        }
        /// Set the value of the `aria-expanded` attribute
        pub fn set_aria_expanded(&mut self, value: bool) {
            self.sys.aria_expanded = value;
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
        /// Get the value of the `aria-level` attribute
        pub fn aria_level(&self) -> std::option::Option<i64> {
            self.sys.aria_level
        }
        /// Set the value of the `aria-level` attribute
        pub fn set_aria_level(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_level = value;
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
        /// Get the value of the `aria-modal` attribute
        pub fn aria_modal(&self) -> bool {
            self.sys.aria_modal
        }
        /// Set the value of the `aria-modal` attribute
        pub fn set_aria_modal(&mut self, value: bool) {
            self.sys.aria_modal = value;
        }
        /// Get the value of the `aria-multiline` attribute
        pub fn aria_multi_line(&self) -> bool {
            self.sys.aria_multi_line
        }
        /// Set the value of the `aria-multiline` attribute
        pub fn set_aria_multi_line(&mut self, value: bool) {
            self.sys.aria_multi_line = value;
        }
        /// Get the value of the `aria-multiselectable` attribute
        pub fn aria_multi_selectable(&self) -> bool {
            self.sys.aria_multi_selectable
        }
        /// Set the value of the `aria-multiselectable` attribute
        pub fn set_aria_multi_selectable(&mut self, value: bool) {
            self.sys.aria_multi_selectable = value;
        }
        /// Get the value of the `aria-orientation` attribute
        pub fn aria_orientation(&self) -> std::option::Option<&str> {
            self.sys.aria_orientation.as_deref()
        }
        /// Set the value of the `aria-orientation` attribute
        pub fn set_aria_orientation(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_orientation = value.map(|v| v.into());
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
        /// Get the value of the `aria-placeholder` attribute
        pub fn aria_placeholder(&self) -> std::option::Option<&str> {
            self.sys.aria_placeholder.as_deref()
        }
        /// Set the value of the `aria-placeholder` attribute
        pub fn set_aria_placeholder(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_placeholder = value.map(|v| v.into());
        }
        /// Get the value of the `aria-posinset` attribute
        pub fn aria_pos_in_set(&self) -> std::option::Option<i64> {
            self.sys.aria_pos_in_set
        }
        /// Set the value of the `aria-posinset` attribute
        pub fn set_aria_pos_in_set(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_pos_in_set = value;
        }
        /// Get the value of the `aria-pressed` attribute
        pub fn aria_pressed(&self) -> std::option::Option<&str> {
            self.sys.aria_pressed.as_deref()
        }
        /// Set the value of the `aria-pressed` attribute
        pub fn set_aria_pressed(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_pressed = value.map(|v| v.into());
        }
        /// Get the value of the `aria-readonly` attribute
        pub fn aria_read_only(&self) -> bool {
            self.sys.aria_read_only
        }
        /// Set the value of the `aria-readonly` attribute
        pub fn set_aria_read_only(&mut self, value: bool) {
            self.sys.aria_read_only = value;
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
        /// Get the value of the `aria-required` attribute
        pub fn aria_required(&self) -> bool {
            self.sys.aria_required
        }
        /// Set the value of the `aria-required` attribute
        pub fn set_aria_required(&mut self, value: bool) {
            self.sys.aria_required = value;
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
        /// Get the value of the `aria-rowcount` attribute
        pub fn aria_row_count(&self) -> std::option::Option<i64> {
            self.sys.aria_row_count
        }
        /// Set the value of the `aria-rowcount` attribute
        pub fn set_aria_row_count(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_row_count = value;
        }
        /// Get the value of the `aria-rowindex` attribute
        pub fn aria_row_index(&self) -> std::option::Option<i64> {
            self.sys.aria_row_index
        }
        /// Set the value of the `aria-rowindex` attribute
        pub fn set_aria_row_index(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_row_index = value;
        }
        /// Get the value of the `aria-rowindextext` attribute
        pub fn aria_row_index_text(&self) -> std::option::Option<&str> {
            self.sys.aria_row_index_text.as_deref()
        }
        /// Set the value of the `aria-rowindextext` attribute
        pub fn set_aria_row_index_text(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_row_index_text = value.map(|v| v.into());
        }
        /// Get the value of the `aria-rowspan` attribute
        pub fn aria_row_span(&self) -> std::option::Option<i64> {
            self.sys.aria_row_span
        }
        /// Set the value of the `aria-rowspan` attribute
        pub fn set_aria_row_span(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_row_span = value;
        }
        /// Get the value of the `aria-selected` attribute
        pub fn aria_selected(&self) -> bool {
            self.sys.aria_selected
        }
        /// Set the value of the `aria-selected` attribute
        pub fn set_aria_selected(&mut self, value: bool) {
            self.sys.aria_selected = value;
        }
        /// Get the value of the `aria-setsize` attribute
        pub fn aria_set_size(&self) -> std::option::Option<i64> {
            self.sys.aria_set_size
        }
        /// Set the value of the `aria-setsize` attribute
        pub fn set_aria_set_size(&mut self, value: std::option::Option<i64>) {
            self.sys.aria_set_size = value;
        }
        /// Get the value of the `aria-sort` attribute
        pub fn aria_sort(&self) -> std::option::Option<&str> {
            self.sys.aria_sort.as_deref()
        }
        /// Set the value of the `aria-sort` attribute
        pub fn set_aria_sort(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_sort = value.map(|v| v.into());
        }
        /// Get the value of the `aria-valuemax` attribute
        pub fn aria_value_max(&self) -> std::option::Option<f64> {
            self.sys.aria_value_max
        }
        /// Set the value of the `aria-valuemax` attribute
        pub fn set_aria_value_max(&mut self, value: std::option::Option<f64>) {
            self.sys.aria_value_max = value;
        }
        /// Get the value of the `aria-valuemin` attribute
        pub fn aria_value_min(&self) -> std::option::Option<f64> {
            self.sys.aria_value_min
        }
        /// Set the value of the `aria-valuemin` attribute
        pub fn set_aria_value_min(&mut self, value: std::option::Option<f64>) {
            self.sys.aria_value_min = value;
        }
        /// Get the value of the `aria-valuenow` attribute
        pub fn aria_value_now(&self) -> std::option::Option<f64> {
            self.sys.aria_value_now
        }
        /// Set the value of the `aria-valuenow` attribute
        pub fn set_aria_value_now(&mut self, value: std::option::Option<f64>) {
            self.sys.aria_value_now = value;
        }
        /// Get the value of the `aria-valuetext` attribute
        pub fn aria_value_text(&self) -> std::option::Option<&str> {
            self.sys.aria_value_text.as_deref()
        }
        /// Set the value of the `aria-valuetext` attribute
        pub fn set_aria_value_text(
            &mut self,
            value: std::option::Option<impl Into<std::borrow::Cow<'static, str>>>,
        ) {
            self.sys.aria_value_text = value.map(|v| v.into());
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
    impl TableFoot {
        /// Access the element's children
        pub fn children(&self) -> &[super::child::TableFootChild] {
            self.children.as_ref()
        }
        /// Mutably access the element's children
        pub fn children_mut(&mut self) -> &mut Vec<super::child::TableFootChild> {
            &mut self.children
        }
    }
    impl crate::Render for TableFoot {
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
    impl std::fmt::Debug for TableFoot {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            crate::Render::render(self, f, 0)?;
            Ok(())
        }
    }
    impl std::fmt::Display for TableFoot {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            for el in &self.children {
                write!(f, "{el}")?;
            }
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableFoot {}
    impl std::convert::Into<html_sys::tables::TableFoot> for TableFoot {
        fn into(self) -> html_sys::tables::TableFoot {
            self.sys
        }
    }
    impl From<html_sys::tables::TableFoot> for TableFoot {
        fn from(sys: html_sys::tables::TableFoot) -> Self {
            Self { sys, children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `TableFoot` element
    #[derive(PartialEq, Clone)]
    pub enum TableFootChild {
        /// The Script element
        Script(crate::generated::all::Script),
        /// The TableRow element
        TableRow(crate::generated::all::TableRow),
        /// The Template element
        Template(crate::generated::all::Template),
    }
    impl std::convert::From<crate::generated::all::Script> for TableFootChild {
        fn from(value: crate::generated::all::Script) -> Self {
            Self::Script(value)
        }
    }
    impl std::convert::From<crate::generated::all::TableRow> for TableFootChild {
        fn from(value: crate::generated::all::TableRow) -> Self {
            Self::TableRow(value)
        }
    }
    impl std::convert::From<crate::generated::all::Template> for TableFootChild {
        fn from(value: crate::generated::all::Template) -> Self {
            Self::Template(value)
        }
    }
    impl crate::Render for TableFootChild {
        fn render(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            match self {
                Self::Script(el) => crate::Render::render(el, f, depth + 1),
                Self::TableRow(el) => crate::Render::render(el, f, depth + 1),
                Self::Template(el) => crate::Render::render(el, f, depth + 1),
            }
        }
    }
    impl std::fmt::Debug for TableFootChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            crate::Render::render(self, f, 0)?;
            Ok(())
        }
    }
    impl std::fmt::Display for TableFootChild {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Script(el) => write!(f, "{el}"),
                Self::TableRow(el) => write!(f, "{el}"),
                Self::Template(el) => write!(f, "{el}"),
            }
        }
    }
}
pub mod builder {
    /// A builder struct for TableFoot
    pub struct TableFootBuilder {
        element: super::element::TableFoot,
    }
    impl TableFootBuilder {
        pub(crate) fn new(element: super::element::TableFoot) -> Self {
            Self { element }
        }
        /// Finish building the element
        pub fn build(&mut self) -> super::element::TableFoot {
            self.element.clone()
        }
        /// Insert a `data-*` property
        pub fn data(
            &mut self,
            data_key: impl Into<std::borrow::Cow<'static, str>>,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut TableFootBuilder {
            self.element.data_map_mut().insert(data_key.into(), value.into());
            self
        }
        /// Insert a class name
        pub fn class(
            &mut self,
            class_name: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut TableFootBuilder {
            self.element.class_set_mut().insert(class_name.into());
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
        /// Append a new `TableRow` element
        pub fn table_row<F>(&mut self, f: F) -> &mut Self
        where
            F: for<'a> FnOnce(
                &'a mut crate::generated::all::builders::TableRowBuilder,
            ) -> &'a mut crate::generated::all::builders::TableRowBuilder,
        {
            let ty: crate::generated::all::TableRow = Default::default();
            let mut ty_builder = crate::generated::all::builders::TableRowBuilder::new(
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
        /// Set the value of the `role` attribute
        pub fn role(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_role(Some(value.into()));
            self
        }
        /// Set the value of the `aria-activedescendant` attribute
        pub fn aria_active_descendant_element(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_active_descendant_element(Some(value.into()));
            self
        }
        /// Set the value of the `aria-atomic` attribute
        pub fn aria_atomic(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_atomic(value);
            self
        }
        /// Set the value of the `aria-autocomplete` attribute
        pub fn aria_auto_complete(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_auto_complete(Some(value.into()));
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
        /// Set the value of the `aria-checked` attribute
        pub fn aria_checked(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_checked(Some(value.into()));
            self
        }
        /// Set the value of the `aria-colcount` attribute
        pub fn aria_col_count(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_col_count(Some(value));
            self
        }
        /// Set the value of the `aria-colindex` attribute
        pub fn aria_col_index(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_col_index(Some(value));
            self
        }
        /// Set the value of the `aria-colindextext` attribute
        pub fn aria_col_index_text(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_col_index_text(Some(value.into()));
            self
        }
        /// Set the value of the `aria-colspan` attribute
        pub fn aria_col_span(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_col_span(Some(value));
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
        /// Set the value of the `aria-expanded` attribute
        pub fn aria_expanded(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_expanded(value);
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
        /// Set the value of the `aria-level` attribute
        pub fn aria_level(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_level(Some(value));
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
        /// Set the value of the `aria-modal` attribute
        pub fn aria_modal(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_modal(value);
            self
        }
        /// Set the value of the `aria-multiline` attribute
        pub fn aria_multi_line(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_multi_line(value);
            self
        }
        /// Set the value of the `aria-multiselectable` attribute
        pub fn aria_multi_selectable(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_multi_selectable(value);
            self
        }
        /// Set the value of the `aria-orientation` attribute
        pub fn aria_orientation(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_orientation(Some(value.into()));
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
        /// Set the value of the `aria-placeholder` attribute
        pub fn aria_placeholder(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_placeholder(Some(value.into()));
            self
        }
        /// Set the value of the `aria-posinset` attribute
        pub fn aria_pos_in_set(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_pos_in_set(Some(value));
            self
        }
        /// Set the value of the `aria-pressed` attribute
        pub fn aria_pressed(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_pressed(Some(value.into()));
            self
        }
        /// Set the value of the `aria-readonly` attribute
        pub fn aria_read_only(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_read_only(value);
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
        /// Set the value of the `aria-required` attribute
        pub fn aria_required(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_required(value);
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
        /// Set the value of the `aria-rowcount` attribute
        pub fn aria_row_count(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_row_count(Some(value));
            self
        }
        /// Set the value of the `aria-rowindex` attribute
        pub fn aria_row_index(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_row_index(Some(value));
            self
        }
        /// Set the value of the `aria-rowindextext` attribute
        pub fn aria_row_index_text(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_row_index_text(Some(value.into()));
            self
        }
        /// Set the value of the `aria-rowspan` attribute
        pub fn aria_row_span(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_row_span(Some(value));
            self
        }
        /// Set the value of the `aria-selected` attribute
        pub fn aria_selected(&mut self, value: bool) -> &mut Self {
            self.element.set_aria_selected(value);
            self
        }
        /// Set the value of the `aria-setsize` attribute
        pub fn aria_set_size(&mut self, value: i64) -> &mut Self {
            self.element.set_aria_set_size(Some(value));
            self
        }
        /// Set the value of the `aria-sort` attribute
        pub fn aria_sort(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_sort(Some(value.into()));
            self
        }
        /// Set the value of the `aria-valuemax` attribute
        pub fn aria_value_max(&mut self, value: f64) -> &mut Self {
            self.element.set_aria_value_max(Some(value));
            self
        }
        /// Set the value of the `aria-valuemin` attribute
        pub fn aria_value_min(&mut self, value: f64) -> &mut Self {
            self.element.set_aria_value_min(Some(value));
            self
        }
        /// Set the value of the `aria-valuenow` attribute
        pub fn aria_value_now(&mut self, value: f64) -> &mut Self {
            self.element.set_aria_value_now(Some(value));
            self
        }
        /// Set the value of the `aria-valuetext` attribute
        pub fn aria_value_text(
            &mut self,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> &mut Self {
            self.element.set_aria_value_text(Some(value.into()));
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
        /// Push a new child element to the list of children.
        pub fn push<T>(&mut self, child_el: T) -> &mut Self
        where
            T: Into<crate::generated::all::children::TableFootChild>,
        {
            let child_el = child_el.into();
            self.element.children_mut().push(child_el);
            self
        }
        /// Extend the list of children with an iterator of child elements.
        pub fn extend<I, T>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = T>,
            T: Into<crate::generated::all::children::TableFootChild>,
        {
            let iter = iter.into_iter().map(|child_el| child_el.into());
            self.element.children_mut().extend(iter);
            self
        }
    }
}
