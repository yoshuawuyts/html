//! Typed HTML encoders.

#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
// #![deny(missing_debug_implementations, nonstandard_style)]
#![warn(future_incompatible, rust_2018_idioms)]
#![warn(missing_docs)]

use std::{borrow::Cow, fmt::Display};

mod generated;
mod manual;

pub use manual::categories::*;

pub use manual::content;
pub use manual::edits;
pub use manual::embedded;
pub use manual::forms;
pub use manual::inline_text;
pub use manual::interactive;
pub use manual::media;
pub use manual::metadata;
pub use manual::root;
pub use manual::scripting;
pub use manual::tables;
pub use manual::text_content;
pub use manual::web_components;

/// An HTML Element
pub trait HtmlElement {}

/// A text element
pub trait TextElement: Display {}
impl<T> HtmlElement for T where T: TextElement {}
impl TextElement for String {}

impl<'a> TextElement for &'a str {}
impl<'a> TextElement for Cow<'a, str> {}
