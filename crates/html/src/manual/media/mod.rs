//! Image and multimedia content
//!
//! HTML supports various multimedia resources such as images, audio, and video.

pub use crate::generated::embedded::{Audio, Image, ImageMap, ImageMapArea, TextTrack, Video};

/// Child elements
pub mod children {
    pub use crate::generated::embedded::children::{
        // AudioChild, ImageChild, ImageMapAreaChild, ImageMapChild, VideoChild,
    };
}
