/// The HTML `<tfoot>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
#[doc(alias = "tfoot")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableFoot {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Describes the role(s) the current element plays in the context of the document.
    pub role: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the currently active element when DOM focus is on a composite widget, combobox, textbox, group, or application.
    pub aria_active_descendant_element: std::option::Option<
        std::borrow::Cow<'static, str>,
    >,
    /// Indicates whether assistive technologies will present all, or only parts of, the changed region based on the change notifications defined by the aria-relevant attribute.
    pub aria_atomic: bool,
    /// Indicates whether inputting text could trigger display of one or more predictions of the user's intended value for a combobox, searchbox, or textbox and specifies how predictions would be presented if they were made.
    pub aria_auto_complete: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a string value that labels the current element, which is intended to be converted into Braille. See related aria-label.
    pub aria_braille_label: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a human-readable, author-localized abbreviated description for the role of an element, which is intended to be converted into Braille. See related aria-roledescription.
    pub aria_braille_role_description: std::option::Option<
        std::borrow::Cow<'static, str>,
    >,
    /// Indicates an element is being modified and that assistive technologies could wait until the modifications are complete before exposing them to the user.
    pub aria_busy: bool,
    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets. See related aria-pressed and aria-selected.
    pub aria_checked: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines the total number of columns in a table, grid, or treegrid. See related aria-colindex.
    pub aria_col_count: std::option::Option<i64>,
    /// Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid. See related aria-colindextext, aria-colcount, and aria-colspan.
    pub aria_col_index: std::option::Option<i64>,
    /// Defines a human readable text alternative of aria-colindex. See related aria-rowindextext.
    pub aria_col_index_text: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid. See related aria-colindex and aria-rowspan.
    pub aria_col_span: std::option::Option<i64>,
    /// Identifies the element (or elements) whose contents or presence are controlled by the current element. See related aria-owns.
    pub aria_controls_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates the element that represents the current item within a container or set of related elements.
    pub aria_current: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that describes the object. See related aria-labelledby and aria-description.
    pub aria_described_by_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a string value that describes or annotates the current element. See related aria-describedby.
    pub aria_description: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that provide additional information related to the object. See related aria-describedby.
    pub aria_details_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable. See related aria-hidden and aria-readonly.
    pub aria_disabled: bool,
    /// [Deprecated in ARIA 1.1] Indicates what functions can be performed when a dragged object is released on the drop target.
    pub aria_drop_effect: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that provides an error message for an object. See related aria-invalid and aria-describedby.
    pub aria_error_message_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether a grouping element owned or controlled by this element is expanded or collapsed.
    pub aria_expanded: bool,
    /// Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion, allows assistive technology to override the general default of reading in document source order.
    pub aria_flow_to_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// [Deprecated in ARIA 1.1] Indicates an element's "grabbed" state in a drag-and-drop operation.
    pub aria_grabbed: bool,
    /// Indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element.
    pub aria_has_popup: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether the element is exposed to an accessibility API. See related aria-disabled.
    pub aria_hidden: bool,
    /// Indicates the entered value does not conform to the format expected by the application. See related aria-errormessage.
    pub aria_invalid: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines keyboard shortcuts that an author has implemented to activate or give focus to an element.
    pub aria_key_shortcuts: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a string value that labels the current element. See related aria-labelledby.
    pub aria_label: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that labels the current element. See related aria-label and aria-describedby.
    pub aria_labelled_by_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines the hierarchical level of an element within a structure.
    pub aria_level: std::option::Option<i64>,
    /// Indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region.
    pub aria_live: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether an element is modal when displayed.
    pub aria_modal: bool,
    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    pub aria_multi_line: bool,
    /// Indicates that the user can select more than one item from the current selectable descendants.
    pub aria_multi_selectable: bool,
    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    pub aria_orientation: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship between DOM elements where the DOM hierarchy cannot be used to represent the relationship. See related aria-controls.
    pub aria_owns_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value. A hint could be a sample value or a brief description of the expected format.
    pub aria_placeholder: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM. See related aria-setsize.
    pub aria_pos_in_set: std::option::Option<i64>,
    /// Indicates the current "pressed" state of toggle buttons. See related aria-checked and aria-selected.
    pub aria_pressed: std::option::Option<std::borrow::Cow<'static, str>>,
    ///  Indicates that the element is not editable, but is otherwise operable. See related aria-disabled.
    pub aria_read_only: bool,
    /// Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified. See related aria-atomic.
    pub aria_relevant: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates that user input is required on the element before a form can be submitted.
    pub aria_required: bool,
    /// Defines a human-readable, author-localized description for the role of an element.
    pub aria_role_description: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines the total number of rows in a table, grid, or treegrid. See related aria-rowindex.
    pub aria_row_count: std::option::Option<i64>,
    /// Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid. See related aria-rowindextext, aria-rowcount, and aria-rowspan.
    pub aria_row_index: std::option::Option<i64>,
    /// Defines a human readable text alternative of aria-rowindex. See related aria-colindextext.
    pub aria_row_index_text: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid. See related aria-rowindex and aria-colspan.
    pub aria_row_span: std::option::Option<i64>,
    /// Indicates the current "selected" state of various widgets. See related aria-checked and aria-pressed.
    pub aria_selected: bool,
    /// Defines the number of items in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM. See related aria-posinset.
    pub aria_set_size: std::option::Option<i64>,
    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    pub aria_sort: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines the maximum allowed value for a range widget.
    pub aria_value_max: std::option::Option<f64>,
    /// Defines the minimum allowed value for a range widget.
    pub aria_value_min: std::option::Option<f64>,
    /// Defines the current value for a range widget. See related aria-valuetext.
    pub aria_value_now: std::option::Option<f64>,
    /// Defines the human readable text alternative of aria-valuenow for a range widget.
    pub aria_value_text: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for TableFoot {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<tfoot")?;
        if let Some(field) = self.role.as_ref() {
            write!(writer, r#" role="{field}""#)?;
        }
        if let Some(field) = self.aria_active_descendant_element.as_ref() {
            write!(writer, r#" aria-activedescendant="{field}""#)?;
        }
        if self.aria_atomic {
            write!(writer, r#" aria-atomic"#)?;
        }
        if let Some(field) = self.aria_auto_complete.as_ref() {
            write!(writer, r#" aria-autocomplete="{field}""#)?;
        }
        if let Some(field) = self.aria_braille_label.as_ref() {
            write!(writer, r#" aria-braillelabel="{field}""#)?;
        }
        if let Some(field) = self.aria_braille_role_description.as_ref() {
            write!(writer, r#" aria-brailleroledescription="{field}""#)?;
        }
        if self.aria_busy {
            write!(writer, r#" aria-busy"#)?;
        }
        if let Some(field) = self.aria_checked.as_ref() {
            write!(writer, r#" aria-checked="{field}""#)?;
        }
        if let Some(field) = self.aria_col_count.as_ref() {
            write!(writer, r#" aria-colcount="{field}""#)?;
        }
        if let Some(field) = self.aria_col_index.as_ref() {
            write!(writer, r#" aria-colindex="{field}""#)?;
        }
        if let Some(field) = self.aria_col_index_text.as_ref() {
            write!(writer, r#" aria-colindextext="{field}""#)?;
        }
        if let Some(field) = self.aria_col_span.as_ref() {
            write!(writer, r#" aria-colspan="{field}""#)?;
        }
        if let Some(field) = self.aria_controls_elements.as_ref() {
            write!(writer, r#" aria-controls="{field}""#)?;
        }
        if let Some(field) = self.aria_current.as_ref() {
            write!(writer, r#" aria-current="{field}""#)?;
        }
        if let Some(field) = self.aria_described_by_elements.as_ref() {
            write!(writer, r#" aria-describedby="{field}""#)?;
        }
        if let Some(field) = self.aria_description.as_ref() {
            write!(writer, r#" aria-description="{field}""#)?;
        }
        if let Some(field) = self.aria_details_elements.as_ref() {
            write!(writer, r#" aria-details="{field}""#)?;
        }
        if self.aria_disabled {
            write!(writer, r#" aria-disabled"#)?;
        }
        if let Some(field) = self.aria_drop_effect.as_ref() {
            write!(writer, r#" aria-dropeffect="{field}""#)?;
        }
        if let Some(field) = self.aria_error_message_elements.as_ref() {
            write!(writer, r#" aria-errormessage="{field}""#)?;
        }
        if self.aria_expanded {
            write!(writer, r#" aria-expanded"#)?;
        }
        if let Some(field) = self.aria_flow_to_elements.as_ref() {
            write!(writer, r#" aria-flowto="{field}""#)?;
        }
        if self.aria_grabbed {
            write!(writer, r#" aria-grabbed"#)?;
        }
        if let Some(field) = self.aria_has_popup.as_ref() {
            write!(writer, r#" aria-haspopup="{field}""#)?;
        }
        if self.aria_hidden {
            write!(writer, r#" aria-hidden"#)?;
        }
        if let Some(field) = self.aria_invalid.as_ref() {
            write!(writer, r#" aria-invalid="{field}""#)?;
        }
        if let Some(field) = self.aria_key_shortcuts.as_ref() {
            write!(writer, r#" aria-keyshortcuts="{field}""#)?;
        }
        if let Some(field) = self.aria_label.as_ref() {
            write!(writer, r#" aria-label="{field}""#)?;
        }
        if let Some(field) = self.aria_labelled_by_elements.as_ref() {
            write!(writer, r#" aria-labelledby="{field}""#)?;
        }
        if let Some(field) = self.aria_level.as_ref() {
            write!(writer, r#" aria-level="{field}""#)?;
        }
        if let Some(field) = self.aria_live.as_ref() {
            write!(writer, r#" aria-live="{field}""#)?;
        }
        if self.aria_modal {
            write!(writer, r#" aria-modal"#)?;
        }
        if self.aria_multi_line {
            write!(writer, r#" aria-multiline"#)?;
        }
        if self.aria_multi_selectable {
            write!(writer, r#" aria-multiselectable"#)?;
        }
        if let Some(field) = self.aria_orientation.as_ref() {
            write!(writer, r#" aria-orientation="{field}""#)?;
        }
        if let Some(field) = self.aria_owns_elements.as_ref() {
            write!(writer, r#" aria-owns="{field}""#)?;
        }
        if let Some(field) = self.aria_placeholder.as_ref() {
            write!(writer, r#" aria-placeholder="{field}""#)?;
        }
        if let Some(field) = self.aria_pos_in_set.as_ref() {
            write!(writer, r#" aria-posinset="{field}""#)?;
        }
        if let Some(field) = self.aria_pressed.as_ref() {
            write!(writer, r#" aria-pressed="{field}""#)?;
        }
        if self.aria_read_only {
            write!(writer, r#" aria-readonly"#)?;
        }
        if let Some(field) = self.aria_relevant.as_ref() {
            write!(writer, r#" aria-relevant="{field}""#)?;
        }
        if self.aria_required {
            write!(writer, r#" aria-required"#)?;
        }
        if let Some(field) = self.aria_role_description.as_ref() {
            write!(writer, r#" aria-roledescription="{field}""#)?;
        }
        if let Some(field) = self.aria_row_count.as_ref() {
            write!(writer, r#" aria-rowcount="{field}""#)?;
        }
        if let Some(field) = self.aria_row_index.as_ref() {
            write!(writer, r#" aria-rowindex="{field}""#)?;
        }
        if let Some(field) = self.aria_row_index_text.as_ref() {
            write!(writer, r#" aria-rowindextext="{field}""#)?;
        }
        if let Some(field) = self.aria_row_span.as_ref() {
            write!(writer, r#" aria-rowspan="{field}""#)?;
        }
        if self.aria_selected {
            write!(writer, r#" aria-selected"#)?;
        }
        if let Some(field) = self.aria_set_size.as_ref() {
            write!(writer, r#" aria-setsize="{field}""#)?;
        }
        if let Some(field) = self.aria_sort.as_ref() {
            write!(writer, r#" aria-sort="{field}""#)?;
        }
        if let Some(field) = self.aria_value_max.as_ref() {
            write!(writer, r#" aria-valuemax="{field}""#)?;
        }
        if let Some(field) = self.aria_value_min.as_ref() {
            write!(writer, r#" aria-valuemin="{field}""#)?;
        }
        if let Some(field) = self.aria_value_now.as_ref() {
            write!(writer, r#" aria-valuenow="{field}""#)?;
        }
        if let Some(field) = self.aria_value_text.as_ref() {
            write!(writer, r#" aria-valuetext="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</tfoot>")?;
        Ok(())
    }
}
impl std::fmt::Display for TableFoot {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for TableFoot {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for TableFoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
