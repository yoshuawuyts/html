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
    fn write_opening_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;

    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;
}
/// The "global attributes" struct
pub struct GlobalAttributes {
    /// Provides a hint for generating a keyboard shortcut for the current element
pub access_key: std::option::Option<String>,
/// Controls whether and how text input is automatically capitalized as it is entered/edited by the user
pub auto_capitalize: std::option::Option<String>,
/// Indicates that an element should be focused on page load, or when the <dialog> that it is part of is displayed
pub autofocus: std::option::Option<String>,
/// A space-separated list of the case-sensitive classes of the element
pub class_: std::option::Option<String>,
/// Indicates if the element should be editable by the user
pub content_editable: std::option::Option<String>,
/// Indicates the directionality of the element's text
pub direction: std::option::Option<String>,
/// Indicates whether the element can be dragged, either with native browser behavior or the HTML Drag and Drop API.
pub draggable: std::option::Option<String>,
/// Defines what action label (or icon) to present for the enter key on virtual keyboards
pub enter_key_hint: std::option::Option<String>,
/// The exportparts global attribute allows you to select and style elements existing in nested shadow trees, by exporting their part names
pub export_parts: std::option::Option<String>,
/// Indicates that the browser should not render the contents of the element
pub hidden: std::option::Option<String>,
/// Defines an identifier (ID) which must be unique in the whole document
pub id: std::option::Option<String>,
/// indicating that the browser will ignore the element
pub inert: std::option::Option<String>,
/// hints at the type of data that might be entered by the user while editing the element or its contents
pub input_mode: std::option::Option<String>,
/// allows you to specify that a standard HTML element should behave like a defined custom built-in element
pub is_: std::option::Option<String>,
/// The itemid global attribute provides microdata in the form of a unique, global identifier of an item
pub item_id: std::option::Option<String>,
/// The itemprop global attribute is used to add properties to an item
pub item_prop: std::option::Option<String>,
/// Properties that are not descendants of an element with the itemscope attribute can be associated with an item using the global attribute itemref
pub item_ref: std::option::Option<String>,
/// itemscope is a boolean global attribute that defines the scope of associated metadata
pub item_scope: std::option::Option<String>,
/// The global attribute itemtype specifies the URL of the vocabulary that will be used to define itemprop's (item properties) in the data structure
pub item_type: std::option::Option<String>,
/// The lang global attribute helps define the language of an element: the language that non-editable elements are written in, or the language that the editable elements should be written in by the user
pub lang: std::option::Option<String>,
/// The nonce global attribute is a content attribute defining a cryptographic nonce ("number used once") which can be used by Content Security Policy to determine whether or not a given fetch will be allowed to proceed for a given element
pub nonce: std::option::Option<String>,
/// The part global attribute contains a space-separated list of the part names of the element
pub part: std::option::Option<String>,
/// The slot global attribute assigns a slot in a shadow DOM shadow tree to an element: An element with a slot attribute is assigned to the slot created by the <slot> element whose name attribute's value matches that slot attribute's value
pub slot: std::option::Option<String>,
/// The spellcheck global attribute is an enumerated attribute that defines whether the element may be checked for spelling errors
pub spellcheck: std::option::Option<String>,
/// The style global attribute contains CSS styling declarations to be applied to the element
pub style: std::option::Option<String>,
/// The tabindex global attribute allows developers to make HTML elements focusable, allow or prevent them from being sequentially focusable (usually with the Tab key, hence the name) and determine their relative ordering for sequential focus navigation
pub tab_index: std::option::Option<String>,
/// The title global attribute contains text representing advisory information related to the element it belongs to
pub title: std::option::Option<String>,
/// The translate global attribute is an enumerated attribute that is used to specify whether an element's translatable attribute values and its Text node children should be translated when the page is localized, or whether to leave them unchanged
pub translate: std::option::Option<String>,

}
