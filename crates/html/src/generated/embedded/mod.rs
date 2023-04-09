//! In addition to regular multimedia content, HTML can include a variety of other content, even if it's not always easy to interact with.
mod embed;
pub use self::embed::element::*;
mod source;
pub use self::source::element::*;
mod area;
pub use self::area::element::*;
mod object;
pub use self::object::element::*;
mod iframe;
pub use self::iframe::element::*;
mod video;
pub use self::video::element::*;
mod img;
pub use self::img::element::*;
mod picture;
pub use self::picture::element::*;
mod track;
pub use self::track::element::*;
mod audio;
pub use self::audio::element::*;
mod map;
pub use self::map::element::*;
/// The various child elements
pub mod children {
    pub use super::embed::child::*;
    pub use super::source::child::*;
    pub use super::area::child::*;
    pub use super::object::child::*;
    pub use super::iframe::child::*;
    pub use super::video::child::*;
    pub use super::img::child::*;
    pub use super::picture::child::*;
    pub use super::track::child::*;
    pub use super::audio::child::*;
    pub use super::map::child::*;
}
