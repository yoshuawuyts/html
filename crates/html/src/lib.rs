//! Typed HTML support for Rust.
//!
//! HTML is not simple. There are many different nodes, attributes,
//! relationships, and types - and no compiler to help with any of that This
//! crate provides type-safe bindings to construct HTML elements natively in
//! Rust. So you don't have to remember whether `<ol>` takes a `<li>`, or whether
//! it's the other way around.
//!
//! # Examples
//!
//! We can create HTML structures one-by-one:
//! ```rust
//! use html::text_content::OrderedList;
//! let tree = OrderedList::builder()
//!     .list_item(|li| li.text("hello").class("pigeon"))
//!     .list_item(|li| li.text("world").class("pigeon"))
//!     .build();
//! let string = tree.to_string();
//! # assert_eq!(string, r#"<ol><li class="pigeon">hello</li><li class="pigeon">world</li></ol>"#);
//! ```
//! But we can also use Rust's native control flow structures such as loops to
//! iterate over items and create HTML:
//! ```rust
//! use html::text_content::OrderedList;
//! let mut ol = OrderedList::builder();
//! for name in ["hello", "world"] {
//!     ol.list_item(|li| li.text(name));
//! }
//! let tree = ol.build();
//! assert_eq!(tree.to_string(), r#"<ol><li>hello</li><li>world</li></ol>"#);
//! ```

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
