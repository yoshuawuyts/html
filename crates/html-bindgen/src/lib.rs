mod generate_sys;
mod parse;
mod scrape;
mod types;

pub use generate_sys::generate;
pub use parse::{parse, Attribute, ParsedNode};
pub use scrape::{scrape_spec, ScrapedNode};
