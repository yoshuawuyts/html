mod generate;
mod parse;
mod scrape;
mod types;

pub use parse::{parse, ParsedNode};
pub use scrape::{scrape_spec, ScrapedNode};
