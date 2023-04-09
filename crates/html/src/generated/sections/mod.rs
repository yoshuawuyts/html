//! Content sectioning elements allow you to organize the document content into logical pieces
mod aside;
pub use self::aside::element::*;
mod nav;
pub use self::nav::element::*;
mod header;
pub use self::header::element::*;
mod h6;
pub use self::h6::element::*;
mod footer;
pub use self::footer::element::*;
mod section;
pub use self::section::element::*;
mod h1;
pub use self::h1::element::*;
mod hgroup;
pub use self::hgroup::element::*;
mod address;
pub use self::address::element::*;
mod h3;
pub use self::h3::element::*;
mod h2;
pub use self::h2::element::*;
mod article;
pub use self::article::element::*;
mod body;
pub use self::body::element::*;
mod h5;
pub use self::h5::element::*;
mod h4;
pub use self::h4::element::*;
/// The various child elements
pub mod children {
    pub use super::aside::child::*;
    pub use super::nav::child::*;
    pub use super::header::child::*;
    pub use super::h6::child::*;
    pub use super::footer::child::*;
    pub use super::section::child::*;
    pub use super::h1::child::*;
    pub use super::hgroup::child::*;
    pub use super::address::child::*;
    pub use super::h3::child::*;
    pub use super::h2::child::*;
    pub use super::article::child::*;
    pub use super::body::child::*;
    pub use super::h5::child::*;
    pub use super::h4::child::*;
}
