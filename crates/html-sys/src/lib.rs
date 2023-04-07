pub mod tables;
pub mod sections;
pub mod scripting;
pub mod edits;
pub mod text;
pub mod embedded;
pub mod root;
pub mod interactive;
pub mod forms;
pub mod metadata;

/// Render an element to a writer.
pub trait RenderElement {
    /// Write the opening tag to a writer.
    fn write_opening_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;

    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;
}