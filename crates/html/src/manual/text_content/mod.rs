//! Text content elements
//!
//! Use HTML text content elements to organize blocks or sections of content
//! placed between the opening <body
//! accessibility and SEO, these elements identify the purpose or structure of
//! that content.

pub use crate::generated::text::{
    BlockQuote, DescriptionDetails, DescriptionList, DescriptionTerm, Division, Figure,
    FigureCaption, ListItem, Menu, OrderedList, Paragraph, PreformattedText, ThematicBreak,
    UnorderedList,
};

/// Child elements
pub mod children {
    pub use crate::generated::text::children::{
        BlockQuoteChild, DescriptionListChild, DivisionChild, FigureChild, MenuChild,
        OrderedListChild, ParagraphChild, PreformattedTextChild, ThematicBreakChild,
        UnorderedListChild,
    };
}
