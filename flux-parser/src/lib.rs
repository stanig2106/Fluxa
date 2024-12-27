// src/lib.rs

pub mod error;
pub mod html;

use crate::html::{parse_html, HtmlDocument};

pub enum ParsedDocument {
    Html(HtmlDocument),
}

pub fn parse_document(input: &str, mime_type: &str) -> Result<ParsedDocument, error::ParserError> {
    match mime_type {
        "text/html" => {
            let doc = parse_html(input)?;
            Ok(ParsedDocument::Html(doc))
        }
        _ => Err(error::ParserError::UnsupportedMimeType(mime_type.to_string())),
    }
}
