//! Metadata contains information about the page.
mod head;
pub use self::head::element::*;
mod style;
pub use self::style::element::*;
mod title;
pub use self::title::element::*;
mod base;
pub use self::base::element::*;
mod link;
pub use self::link::element::*;
mod meta;
pub use self::meta::element::*;
/// The various child elements
pub mod children {
    pub use super::head::child::*;
    pub use super::style::child::*;
    pub use super::title::child::*;
    pub use super::base::child::*;
    pub use super::link::child::*;
    pub use super::meta::child::*;
}
