//! In order to create dynamic content and Web applications, HTML supports the use of scripting languages
mod canvas;
pub use canvas::*;
mod noscript;
pub use noscript::*;
mod script;
pub use script::*;
mod slot;
pub use slot::*;
mod template;
pub use template::*;
