//! Scripting elements
//!
//! In order to create dynamic content and Web applications, HTML supports the
//! use of scripting languages, most prominently JavaScript. Certain elements
//! support this capability.

pub use crate::generated::scripting::{Canvas, NoScript, Script};

/// Child elements
pub mod children {
    pub use crate::generated::scripting::children::{CanvasChild, NoScriptChild, ScriptChild};
}
