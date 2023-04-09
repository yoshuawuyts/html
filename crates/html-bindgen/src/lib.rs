pub mod generate;
pub mod merge;
pub mod parse;
pub mod scrape;
pub(crate) mod utils;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;
