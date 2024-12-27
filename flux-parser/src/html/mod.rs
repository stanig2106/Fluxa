use crate::error::ParserError;
pub use crate::html::parser::HtmlDocument;
use crate::html::token::tokenize;

mod parser;
mod token;

pub fn parse_html(input: &str) -> Result<HtmlDocument, ParserError> {
    // 1. Tokenize the input
    let tokens = tokenize(input)?;

    // 2. Parse the tokens into a DOM tree
    let mut parser = crate::html::parser::HtmlParser::new(tokens);
    let nodes = parser.parse_nodes()?;

    // 3. Renvoie un HtmlDocument
    Ok(HtmlDocument { root_nodes: nodes })
}
