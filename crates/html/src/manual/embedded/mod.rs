//! Embedded content elements
//!
//! In addition to regular multimedia contentChild, HTML can include a variety of other contentChild, even if it's not always easy to interact with.

pub use crate::generated::embedded::{Embed, Iframe, MediaSource, Object, Picture};

/// Child elements
pub mod children {
    pub use crate::generated::embedded::children::{EmbedChild, IframeChild, ObjectChild};
}
