//! In order to create dynamic content and Web applications, HTML supports the use of scripting languages
mod canvas;
pub use canvas::*;
mod script;
pub use script::*;
mod template;
pub use template::*;
mod slot;
pub use slot::*;
mod noscript;
pub use noscript::*;
