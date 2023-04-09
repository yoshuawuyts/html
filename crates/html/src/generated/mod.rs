//! HTML elements support
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
/// All auto-generated items in this crate
pub(crate) mod all {
    pub(crate) use super::edits::*;
    pub(crate) use super::embedded::*;
    pub(crate) use super::forms::*;
    pub(crate) use super::interactive::*;
    pub(crate) use super::metadata::*;
    pub(crate) use super::root::*;
    pub(crate) use super::scripting::*;
    pub(crate) use super::sections::*;
    pub(crate) use super::tables::*;
    pub(crate) use super::text::*;
}
