//! HTML offers a selection of elements which help to create interactive user interface objects.
mod summary;
pub use self::summary::element::*;
mod dialog;
pub use self::dialog::element::*;
mod details;
pub use self::details::element::*;
/// The various child elements
pub mod children {
    pub use super::summary::child::*;
    pub use super::dialog::child::*;
    pub use super::details::child::*;
}
