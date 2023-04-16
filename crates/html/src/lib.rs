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
//! # #![allow(unused)]
//! use html::text_content::OrderedList;
//! let tree = OrderedList::builder()
//!     .list_item(|li| li.text("hello").class("pigeon"))
//!     .list_item(|li| li.text("world").class("pigeon"))
//!     .build();
//! let string = tree.to_string();
//! ```
//! But we can also use Rust's native control flow structures such as loops to
//! iterate over items and create HTML:
//! ```rust
//! # #![allow(unused)]
//! use html::text_content::OrderedList;
//! let mut ol = OrderedList::builder();
//! for name in ["hello", "world"] {
//!     ol.list_item(|li| li.text(name));
//! }
//! let tree = ol.build();
//! ```

#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
// #![deny(missing_debug_implementations, nonstandard_style)]
#![warn(future_incompatible, rust_2018_idioms)]
#![warn(missing_docs)]

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

/// Render an HTML element to a string.
///
/// This API is similar to `Display`, but it takes a `depth` argument which
/// allows rendered items to be indented.
///
/// Users of this crate are expected to keep using the `Display` interface as
/// normal. This trait only exists for internal bookkeeping.
pub trait Render {
    /// Render an element with a given `depth` argument.
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result;
}

impl Render for Cow<'static, str> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        write!(f, "{:level$}", "", level = depth * 4)?;
        std::fmt::Display::fmt(self, f)
    }
}

impl<T> Render for &T
where
    T: Render + ?Sized,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        Render::render(&**self, f, depth)
    }
}
impl<T> Render for &mut T
where
    T: Render + ?Sized,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        Render::render(&**self, f, depth)
    }
}

/// An HTML Element
pub trait HtmlElement {}
