//! HTML content categories
//!
//! Each element in HTML falls into zero or more categories that group elements
//! with similar characteristics together.
//!
//! # References
//!
//! - [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Content_categories)
//! - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#kinds-of-content)

/// The Metadata content category
///
/// Metadata content is content that sets up the presentation or behavior of the
/// rest of the content, or that sets up the relationship of the document with
/// other documents, or that conveys other "out of band" information.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#metadata-content)
pub trait MetadataContent {}

/// The Flow content category
///
/// Most elements that are used in the body of documents and applications are
/// categorized as flow content.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#flow-content)
pub trait FlowContent {}

/// The Sectioning content category
///
/// Sectioning content is content that defines the scope of header and footer
/// elements.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#sectioning-content)
pub trait SectioningContent: FlowContent {}

/// The Heading content category
///
/// Heading content defines the heading of a section (whether explicitly marked
/// up using sectioning content elements, or implied by the heading content
/// itself).
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#heading-content)
pub trait HeadingContent: FlowContent {}

/// The Phrasing content category
///
/// Phrasing content is the text of the document, as well as elements that mark
/// up that text at the intra-paragraph level. Runs of phrasing content form
/// paragraphs.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#phrasing-content)
pub trait PhrasingContent: FlowContent {}

/// The Embedded content category
///
/// Embedded content is content that imports another resource into the document,
/// or content from another vocabulary that is inserted into the document.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#embedded-content)
pub trait EmbeddedContent: PhrasingContent {}

/// The Interactive content category
///
/// Interactive content is content that is specifically intended for user
/// interaction.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#interactive-content)
pub trait InteractiveContent: FlowContent {}

/// The Palpable content category
///
/// As a general rule, elements whose content model allows any flow content or
/// phrasing content should have at least one node in its contents that is
/// palpable content and that does not have the hidden attribute specified.
///
/// This requirement is not a hard requirement, however, as there are many cases
/// where an element can be empty legitimately, for example when it is used as a
/// placeholder which will later be filled in by a script, or when the element
/// is part of a template and would on most pages be filled in but on some pages
/// is not relevant.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#palpable-content)
pub trait PalpableContent {}

/// The Script-supporting elements content category
///
/// Script-supporting elements are those that do not represent anything
/// themselves (i.e. they are not rendered), but are used to support scripts,
/// e.g. to provide functionality for the user.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#script-supporting-elements)
pub trait ScriptSupportingContent {}

/// The Transparent content category
///
/// Some elements are described as transparent; they have "transparent" in the
/// description of their content model. The content model of a transparent
/// element is derived from the content model of its parent element: the
/// elements required in the part of the content model that is "transparent" are
/// the same elements as required in the part of the content model of the parent
/// of the transparent element in which the transparent element finds itself.
///
/// # References
///
/// - [HTML Specification](https://html.spec.whatwg.org/multipage/dom.html#transparent-content-models)
pub trait TransparentContent {}
