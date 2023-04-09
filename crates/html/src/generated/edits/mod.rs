//! These elements let you provide indications that specific parts of the text have been altered.
mod del;
pub use self::del::element::*;
mod ins;
pub use self::ins::element::*;
/// The various child elements
pub mod children {
    pub use super::del::child::*;
    pub use super::ins::child::*;
}
