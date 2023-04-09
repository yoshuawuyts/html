//! In order to create dynamic content and Web applications, HTML supports the use of scripting languages
mod canvas;
pub use self::canvas::element::*;
mod script;
pub use self::script::element::*;
mod template;
pub use self::template::element::*;
mod slot;
pub use self::slot::element::*;
mod noscript;
pub use self::noscript::element::*;
/// The various child elements
pub mod children {
    pub use super::canvas::child::*;
    pub use super::script::child::*;
    pub use super::template::child::*;
    pub use super::slot::child::*;
    pub use super::noscript::child::*;
}
