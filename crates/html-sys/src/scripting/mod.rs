//! In order to create dynamic content and Web applications, HTML supports the use of scripting languages
mod script;
pub use script::*;
mod noscript;
pub use noscript::*;
mod slot;
pub use slot::*;
mod canvas;
pub use canvas::*;
mod template;
pub use template::*;
