//! HTML provides a number of elements which can be used together to create forms which the user can fill out and submit to the website or application.
mod input;
pub use self::input::element::*;
mod meter;
pub use self::meter::element::*;
mod output;
pub use self::output::element::*;
mod fieldset;
pub use self::fieldset::element::*;
mod button;
pub use self::button::element::*;
mod optgroup;
pub use self::optgroup::element::*;
mod option;
pub use self::option::element::*;
mod textarea;
pub use self::textarea::element::*;
mod progress;
pub use self::progress::element::*;
mod datalist;
pub use self::datalist::element::*;
mod label;
pub use self::label::element::*;
mod form;
pub use self::form::element::*;
mod select;
pub use self::select::element::*;
mod legend;
pub use self::legend::element::*;
/// The various child elements
pub mod children {
    pub use super::input::child::*;
    pub use super::meter::child::*;
    pub use super::output::child::*;
    pub use super::fieldset::child::*;
    pub use super::button::child::*;
    pub use super::optgroup::child::*;
    pub use super::option::child::*;
    pub use super::textarea::child::*;
    pub use super::progress::child::*;
    pub use super::datalist::child::*;
    pub use super::label::child::*;
    pub use super::form::child::*;
    pub use super::select::child::*;
    pub use super::legend::child::*;
}
