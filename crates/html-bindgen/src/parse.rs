use crate::types;
use crate::ScrapedNode;

/// The parsed values converted from the raw spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedNode {
    pub tag_name: String,
}

pub fn parse(
    scraped: impl Iterator<Item = types::Result<ScrapedNode>>,
) -> types::Result<Vec<ParsedNode>> {
    for scraped in scraped {
        let scraped = scraped?;
        dbg!(scraped);
    }
    todo!()
}
