pub mod edits;
pub mod embedded;
pub mod forms;
pub mod interactive;
pub mod metadata;
pub mod root;
pub mod scripting;
pub mod sections;
pub mod tables;
pub mod text;
/// Render an element to a writer.
pub trait RenderElement {
    /// Write the opening tag to a writer.
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result;
    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result;
}
/// Container for `data-*` attributes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DataMap {
    map: std::collections::HashMap<
        std::borrow::Cow<'static, str>,
        std::borrow::Cow<'static, str>,
    >,
}
impl std::ops::Deref for DataMap {
    type Target = std::collections::HashMap<
        std::borrow::Cow<'static, str>,
        std::borrow::Cow<'static, str>,
    >;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl std::ops::DerefMut for DataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
impl std::fmt::Display for DataMap {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.map.iter() {
            write!(writer, r#" data-{key}="{value}""#)?;
        }
        Ok(())
    }
}
/// The "global attributes" struct
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GlobalAttributes {
    /// Provides a hint for generating a keyboard shortcut for the current element
    pub access_key: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Controls whether and how text input is automatically capitalized as it is entered/edited by the user
    pub auto_capitalize: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates that an element should be focused on page load, or when the <dialog> that it is part of is displayed
    pub autofocus: bool,
    /// A space-separated list of the case-sensitive classes of the element
    pub class: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates if the element should be editable by the user
    pub content_editable: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates the directionality of the element's text
    pub direction: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether the element can be dragged, either with native browser behavior or the HTML Drag and Drop API.
    pub draggable: bool,
    /// Defines what action label (or icon) to present for the enter key on virtual keyboards
    pub enter_key_hint: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The exportparts global attribute allows you to select and style elements existing in nested shadow trees, by exporting their part names
    pub export_parts: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates that the browser should not render the contents of the element
    pub hidden: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines an identifier (ID) which must be unique in the whole document
    pub id: std::option::Option<std::borrow::Cow<'static, str>>,
    /// indicating that the browser will ignore the element
    pub inert: bool,
    /// hints at the type of data that might be entered by the user while editing the element or its contents
    pub input_mode: std::option::Option<std::borrow::Cow<'static, str>>,
    /// allows you to specify that a standard HTML element should behave like a defined custom built-in element
    pub is_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The itemid global attribute provides microdata in the form of a unique, global identifier of an item
    pub item_id: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The itemprop global attribute is used to add properties to an item
    pub item_prop: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Properties that are not descendants of an element with the itemscope attribute can be associated with an item using the global attribute itemref
    pub item_ref: std::option::Option<std::borrow::Cow<'static, str>>,
    /// itemscope is a boolean global attribute that defines the scope of associated metadata
    pub item_scope: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The global attribute itemtype specifies the URL of the vocabulary that will be used to define itemprop's (item properties) in the data structure
    pub item_type: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The lang global attribute helps define the language of an element: the language that non-editable elements are written in, or the language that the editable elements should be written in by the user
    pub lang: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The nonce global attribute is a content attribute defining a cryptographic nonce ("number used once") which can be used by Content Security Policy to determine whether or not a given fetch will be allowed to proceed for a given element
    pub nonce: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The part global attribute contains a space-separated list of the part names of the element
    pub part: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The slot global attribute assigns a slot in a shadow DOM shadow tree to an element: An element with a slot attribute is assigned to the slot created by the <slot> element whose name attribute's value matches that slot attribute's value
    pub slot: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The spellcheck global attribute is an enumerated attribute that defines whether the element may be checked for spelling errors
    pub spellcheck: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The style global attribute contains CSS styling declarations to be applied to the element
    pub style: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The tabindex global attribute allows developers to make HTML elements focusable, allow or prevent them from being sequentially focusable (usually with the Tab key, hence the name) and determine their relative ordering for sequential focus navigation
    pub tab_index: std::option::Option<i64>,
    /// The title global attribute contains text representing advisory information related to the element it belongs to
    pub title: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The translate global attribute is an enumerated attribute that is used to specify whether an element's translatable attribute values and its Text node children should be translated when the page is localized, or whether to leave them unchanged
    pub translate: bool,
}
impl std::fmt::Display for GlobalAttributes {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(field) = self.access_key.as_ref() {
            write!(writer, r#" accesskey="{field}""#)?;
        }
        if let Some(field) = self.auto_capitalize.as_ref() {
            write!(writer, r#" autocapitalize="{field}""#)?;
        }
        if self.autofocus {
            write!(writer, r#" autofocus"#)?;
        }
        if let Some(field) = self.class.as_ref() {
            write!(writer, r#" class="{field}""#)?;
        }
        if let Some(field) = self.content_editable.as_ref() {
            write!(writer, r#" contenteditable="{field}""#)?;
        }
        if let Some(field) = self.direction.as_ref() {
            write!(writer, r#" dir="{field}""#)?;
        }
        if self.draggable {
            write!(writer, r#" draggable"#)?;
        }
        if let Some(field) = self.enter_key_hint.as_ref() {
            write!(writer, r#" enterkeyhint="{field}""#)?;
        }
        if let Some(field) = self.export_parts.as_ref() {
            write!(writer, r#" exportparts="{field}""#)?;
        }
        if let Some(field) = self.hidden.as_ref() {
            write!(writer, r#" hidden="{field}""#)?;
        }
        if let Some(field) = self.id.as_ref() {
            write!(writer, r#" id="{field}""#)?;
        }
        if self.inert {
            write!(writer, r#" inert"#)?;
        }
        if let Some(field) = self.input_mode.as_ref() {
            write!(writer, r#" inputmode="{field}""#)?;
        }
        if let Some(field) = self.is_.as_ref() {
            write!(writer, r#" is="{field}""#)?;
        }
        if let Some(field) = self.item_id.as_ref() {
            write!(writer, r#" itemid="{field}""#)?;
        }
        if let Some(field) = self.item_prop.as_ref() {
            write!(writer, r#" itemprop="{field}""#)?;
        }
        if let Some(field) = self.item_ref.as_ref() {
            write!(writer, r#" itemref="{field}""#)?;
        }
        if let Some(field) = self.item_scope.as_ref() {
            write!(writer, r#" itemscope="{field}""#)?;
        }
        if let Some(field) = self.item_type.as_ref() {
            write!(writer, r#" itemtype="{field}""#)?;
        }
        if let Some(field) = self.lang.as_ref() {
            write!(writer, r#" lang="{field}""#)?;
        }
        if let Some(field) = self.nonce.as_ref() {
            write!(writer, r#" nonce="{field}""#)?;
        }
        if let Some(field) = self.part.as_ref() {
            write!(writer, r#" part="{field}""#)?;
        }
        if let Some(field) = self.slot.as_ref() {
            write!(writer, r#" slot="{field}""#)?;
        }
        if let Some(field) = self.spellcheck.as_ref() {
            write!(writer, r#" spellcheck="{field}""#)?;
        }
        if let Some(field) = self.style.as_ref() {
            write!(writer, r#" style="{field}""#)?;
        }
        if let Some(field) = self.tab_index.as_ref() {
            write!(writer, r#" tabindex="{field}""#)?;
        }
        if let Some(field) = self.title.as_ref() {
            write!(writer, r#" title="{field}""#)?;
        }
        if self.translate {
            write!(writer, r#" translate"#)?;
        }
        Ok(())
    }
}
