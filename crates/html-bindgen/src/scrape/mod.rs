mod aria;
mod elements;
mod webidls;

pub use aria::{
    scrape_aria, scrape_html_aria, ScrapedAriaElement, ScrapedAriaProperty, ScrapedAriaRole,
};
pub use elements::{scrape_elements, ScrapedElement};
pub use webidls::scrape_webidls;
