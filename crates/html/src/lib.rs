//! Structured HTML parser and encoder.
//!
#![forbid(unsafe_code)]
// #![deny(missing_debug_implementations, nonstandard_style)]
#![warn(future_incompatible, rust_2018_idioms)]
#![warn(missing_docs, unreachable_pub)]

use std::{borrow::Cow, fmt::Display};

pub mod elements;

/// An HTML Element
pub trait HtmlElement: Display {}

/// A text element
pub trait TextElement: Display {}
impl<T> HtmlElement for T where T: TextElement {}
impl TextElement for String {}

impl<'a> TextElement for &'a str {}
impl<'a> TextElement for Cow<'a, str> {}

mod heading;

pub use heading::{Heading, HeadingLevel};
