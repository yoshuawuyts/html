//! Inline text elements
//!
//! Use the HTML inline text semantic to define the meaning, structure, or style
//! of a word, line, or any arbitrary piece of text.

pub use crate::generated::text::{
    Abbreviation, Anchor, BidirectionalIsolate, BidirectionalTextOverride, Bold, Cite, Code, Data,
    Definition, Emphasis, Italic, KeyboardInput, LineBreak, LineBreakOpportunity, MarkText,
    Quotation, RubyAnnotation, RubyFallbackParenthesis, RubyText, SampleOutput, SideComment, Span,
    StrikeThrough, Strong, SubScript, SuperScript, Time, Underline, Variable,
};

/// Child elements
pub mod children {
    pub use crate::generated::text::children::{
        AbbreviationChild, BidirectionalIsolateChild, BidirectionalTextOverrideChild, BoldChild,
        CiteChild, CodeChild, DataChild, EmphasisChild, ItalicChild, KeyboardInputChild,
        MarkTextChild, QuotationChild, SampleOutputChild, SideCommentChild, SpanChild,
        StrikeThroughChild, StrongChild, SubScriptChild, SuperScriptChild, UnderlineChild,
        VariableChild,
    };
}
