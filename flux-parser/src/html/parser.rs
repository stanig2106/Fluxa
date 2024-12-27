/// html_parser.rs

use crate::error::ParserError;
use crate::html::token::{tokenize, Token};

#[derive(Debug)]
pub struct HtmlDocument {
    pub root_nodes: Vec<HtmlNode>,
}

#[derive(Debug)]
pub enum HtmlNode {
    Element(HtmlElement),
    Text(String),
    Comment(String),
}

#[derive(Debug)]
pub struct HtmlElement {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<HtmlNode>,
}

/// Fonction principale de parsing HTML
pub fn parse_html(input: &str) -> Result<HtmlDocument, ParserError> {
    // 1. Tokenize the input
    let tokens = tokenize(input)?;

    // 2. Parse the tokens into a DOM tree
    let mut parser = HtmlParser::new(tokens);
    let nodes = parser.parse_nodes()?;

    // 3. Renvoie un HtmlDocument
    Ok(HtmlDocument { root_nodes: nodes })
}

/// A small struct to hold state for our parser
struct HtmlParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl HtmlParser {
    fn new(tokens: Vec<Token>) -> Self {
        HtmlParser { tokens, pos: 0 }
    }

    /// Parse multiple top-level nodes
    fn parse_nodes(&mut self) -> Result<Vec<HtmlNode>, ParserError> {
        let mut nodes = Vec::new();
        loop {
            match self.current_token()? {
                Token::Eof => break,
                _ => {
                    // Attempt to parse a node
                    let node = self.parse_node()?;
                    nodes.push(node);
                }
            }
        }
        Ok(nodes)
    }

    /// Parse a single node: could be an element, comment, or text
    fn parse_node(&mut self) -> Result<HtmlNode, ParserError> {
        match self.current_token()? {
            Token::StartTag(_, _) => Ok(HtmlNode::Element(self.parse_element()?)),
            Token::Comment(text) => {
                let comment_text = text.clone();
                self.advance()?; // consume the Token::Comment
                Ok(HtmlNode::Comment(comment_text))
            }
            Token::Text(text) => {
                let text_content = text.clone();
                self.advance()?; // consume the Token::Text
                Ok(HtmlNode::Text(text_content))
            }
            other => Err(ParserError::Other(format!(
                "Unexpected token in parse_node: {:?}",
                other
            ))),
        }
    }

    /// Parse an element: assumes current token is Token::StartTag
    fn parse_element(&mut self) -> Result<HtmlElement, ParserError> {
        // e.g. <div ...>
        let (tag_name, attributes) = match self.current_token()? {
            Token::StartTag(name, attrs) => (name.clone(), attrs.clone()),
            other => {
                return Err(ParserError::Other(format!(
                    "Expected StartTag, found: {:?}",
                    other
                )))
            }
        };
        // consume start tag
        self.advance()?;

        // parse children until we see a matching end tag or EOF
        let children = self.parse_children(&tag_name)?;

        // Expect an EndTag
        match self.current_token()? {
            Token::EndTag(end_name) if *end_name == tag_name => {
                self.advance()?; // consume </tag_name>
            }
            Token::EndTag(end_name) => {
                return Err(ParserError::Other(format!(
                    "Expected </{}>, but found </{}>",
                    tag_name, end_name
                )));
            }
            other => {
                return Err(ParserError::Other(format!(
                    "Expected end tag </{}>, found: {:?}",
                    tag_name, other
                )));
            }
        }

        Ok(HtmlElement {
            tag_name,
            attributes,
            children,
        })
    }

    /// Parse children (HtmlNode) until we see a matching `</tag_name>` or EOF
    fn parse_children(&mut self, parent_tag: &str) -> Result<Vec<HtmlNode>, ParserError> {
        let mut children = Vec::new();

        loop {
            match self.current_token()? {
                Token::EndTag(end_name) if end_name == parent_tag => {
                    // Stop, let parse_element handle this end tag
                    break;
                }
                Token::Eof => {
                    // No more tokens; might be an unclosed parent, but let's just break
                    break;
                }
                _ => {
                    let child = self.parse_node()?;
                    children.push(child);
                }
            }
        }

        Ok(children)
    }

    /// Returns the current token without consuming it
    fn current_token(&self) -> Result<&Token, ParserError> {
        self.tokens.get(self.pos).ok_or_else(|| {
            ParserError::Other("Attempted to read beyond end of tokens".to_string())
        })
    }

    /// Advances the parser to the next token
    fn advance(&mut self) -> Result<(), ParserError> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        Ok(())
    }
}
