//! The elements here are used to create and handle tabular data
mod tr;
pub use self::tr::element::*;
mod tbody;
pub use self::tbody::element::*;
mod colgroup;
pub use self::colgroup::element::*;
mod caption;
pub use self::caption::element::*;
mod tfoot;
pub use self::tfoot::element::*;
mod col;
pub use self::col::element::*;
mod table;
pub use self::table::element::*;
mod th;
pub use self::th::element::*;
mod thead;
pub use self::thead::element::*;
mod td;
pub use self::td::element::*;
/// The various child elements
pub mod children {
    pub use super::tr::child::*;
    pub use super::tbody::child::*;
    pub use super::colgroup::child::*;
    pub use super::caption::child::*;
    pub use super::tfoot::child::*;
    pub use super::col::child::*;
    pub use super::table::child::*;
    pub use super::th::child::*;
    pub use super::thead::child::*;
    pub use super::td::child::*;
}
