/** The HTML `<section>` element

 [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)*/
#[doc(alias = "section")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Section {
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
    /// Defines a string value that labels the current element, which is intended to be converted into Braille. See related aria-label.
    pub aria_braille_label: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a human-readable, author-localized abbreviated description for the role of an element, which is intended to be converted into Braille. See related aria-roledescription.
    pub aria_braille_role_description: std::option::Option<
        std::borrow::Cow<'static, str>,
    >,
    /// Indicates an element is being modified and that assistive technologies could wait until the modifications are complete before exposing them to the user.
    pub aria_busy: bool,
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
    /// Indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region.
    pub aria_live: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether an element is modal when displayed.
    pub aria_modal: bool,
    /// Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship between DOM elements where the DOM hierarchy cannot be used to represent the relationship. See related aria-controls.
    pub aria_owns_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified. See related aria-atomic.
    pub aria_relevant: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a human-readable, author-localized description for the role of an element.
    pub aria_role_description: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Section {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "{}<{}", "", "section")?;
        if let Some(field) = self.role.as_ref() {
            write!(writer, r#" {}="{field}""#, "role")?;
        }
        if let Some(field) = self.aria_active_descendant_element.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-activedescendant")?;
        }
        if self.aria_atomic {
            write!(writer, " {}", "aria-atomic")?;
        }
        if let Some(field) = self.aria_braille_label.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-braillelabel")?;
        }
        if let Some(field) = self.aria_braille_role_description.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-brailleroledescription")?;
        }
        if self.aria_busy {
            write!(writer, " {}", "aria-busy")?;
        }
        if let Some(field) = self.aria_controls_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-controls")?;
        }
        if let Some(field) = self.aria_current.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-current")?;
        }
        if let Some(field) = self.aria_described_by_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-describedby")?;
        }
        if let Some(field) = self.aria_description.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-description")?;
        }
        if let Some(field) = self.aria_details_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-details")?;
        }
        if self.aria_disabled {
            write!(writer, " {}", "aria-disabled")?;
        }
        if let Some(field) = self.aria_drop_effect.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-dropeffect")?;
        }
        if let Some(field) = self.aria_error_message_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-errormessage")?;
        }
        if self.aria_expanded {
            write!(writer, " {}", "aria-expanded")?;
        }
        if let Some(field) = self.aria_flow_to_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-flowto")?;
        }
        if self.aria_grabbed {
            write!(writer, " {}", "aria-grabbed")?;
        }
        if let Some(field) = self.aria_has_popup.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-haspopup")?;
        }
        if self.aria_hidden {
            write!(writer, " {}", "aria-hidden")?;
        }
        if let Some(field) = self.aria_invalid.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-invalid")?;
        }
        if let Some(field) = self.aria_key_shortcuts.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-keyshortcuts")?;
        }
        if let Some(field) = self.aria_label.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-label")?;
        }
        if let Some(field) = self.aria_labelled_by_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-labelledby")?;
        }
        if let Some(field) = self.aria_live.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-live")?;
        }
        if self.aria_modal {
            write!(writer, " {}", "aria-modal")?;
        }
        if let Some(field) = self.aria_owns_elements.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-owns")?;
        }
        if let Some(field) = self.aria_relevant.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-relevant")?;
        }
        if let Some(field) = self.aria_role_description.as_ref() {
            write!(writer, r#" {}="{field}""#, "aria-roledescription")?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</{}>", "section")?;
        Ok(())
    }
}
impl std::fmt::Display for Section {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Section {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Section {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
