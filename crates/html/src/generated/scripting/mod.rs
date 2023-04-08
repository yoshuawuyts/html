//! In order to create dynamic content and Web applications, HTML supports the use of scripting languages
mod canvas;
pub use self::canvas::*;
mod script;
pub use self::script::*;
mod template;
pub use self::template::*;
mod slot;
pub use self::slot::*;
mod noscript;
pub use self::noscript::*;
