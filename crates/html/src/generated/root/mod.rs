//! Represents the root (top-level element) of an HTML document
mod html;
pub use self::html::element::*;
/// The various child elements
pub mod children {
    pub use super::html::child::*;
}
