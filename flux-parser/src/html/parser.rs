/// html_parser.rs

use crate::error::ParserError;
use crate::html::token::Token;


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

impl HtmlElement {
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
    }
}

/// Fonction principale de parsing HTML

/// A small struct to hold state for our parser
pub(crate) struct HtmlParser {
    tokens: Vec<Token>,
    pos: usize,
}

const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input",
    "link", "meta", "param", "source", "track", "wbr", "!doctype",
];

impl HtmlParser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        HtmlParser { tokens, pos: 0 }
    }

    /// Parse multiple top-level nodes
    pub(crate) fn parse_nodes(&mut self) -> Result<Vec<HtmlNode>, ParserError> {
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
        // e.g., <div ...>
        let (tag_name, attributes) = match self.current_token()? {
            Token::StartTag(name, attrs) => (name.clone(), attrs.clone()),
            other => {
                return Err(ParserError::Other(format!(
                    "Expected StartTag, found: {:?}",
                    other
                )))
            }
        };

        // Consume start tag
        self.advance()?;

        // Check if the tag is a void element
        if VOID_ELEMENTS.contains(&tag_name.to_lowercase().as_str()) {
            // Void elements do not have children and do not require end tags
            return Ok(HtmlElement {
                tag_name,
                attributes,
                children: Vec::new(),
            });
        }

        // Parse children until we see a matching end tag or EOF
        let children = self.parse_children(&tag_name)?;

        // Expect an EndTag
        match self.current_token()? {
            Token::EndTag(ref end_name) if end_name.eq_ignore_ascii_case(&tag_name) => {
                self.advance()?; // Consume </tag_name>
            }
            Token::EndTag(ref end_name) => {
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
            tag_name: tag_name.to_lowercase(),
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


#[cfg(test)]
mod tests {
    use super::*;
    // imports from the same module where parse_html is defined
    use crate::error::ParserError;
    use crate::html::parse_html;

    #[test]
    fn test_parse_empty_input() -> Result<(), ParserError> {
        let input = "";
        let doc = parse_html(input)?;
        assert!(doc.root_nodes.is_empty(), "Root nodes should be empty for empty input");
        Ok(())
    }

    #[test]
    fn test_parse_single_element_no_children() -> Result<(), ParserError> {
        let input = "<div></div>";
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 1);
        match &doc.root_nodes[0] {
            HtmlNode::Element(element) => {
                assert_eq!(element.tag_name, "div");
                assert!(element.children.is_empty(), "Expected no children for <div></div>");
            }
            _ => panic!("Expected a single element node"),
        }
        Ok(())
    }

    #[test]
    fn test_parse_single_element_with_text_child() -> Result<(), ParserError> {
        let input = "<div>Hello World</div>";
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 1);

        match &doc.root_nodes[0] {
            HtmlNode::Element(element) => {
                // Check tag name
                assert_eq!(element.tag_name, "div");
                // Expect one child node of type Text
                assert_eq!(element.children.len(), 1);

                match &element.children[0] {
                    HtmlNode::Text(text) => {
                        assert_eq!(text, "Hello World");
                    }
                    _ => panic!("Expected a text node child"),
                }
            }
            _ => panic!("Expected a single element node"),
        }
        Ok(())
    }

    #[test]
    fn test_parse_nested_elements() -> Result<(), ParserError> {
        let input = "<div><span>Test</span></div>";
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 1);

        match &doc.root_nodes[0] {
            HtmlNode::Element(element) => {
                assert_eq!(element.tag_name, "div");
                assert_eq!(element.children.len(), 1);

                match &element.children[0] {
                    HtmlNode::Element(child_elem) => {
                        assert_eq!(child_elem.tag_name, "span");
                        assert_eq!(child_elem.children.len(), 1);

                        match &child_elem.children[0] {
                            HtmlNode::Text(text) => {
                                assert_eq!(text, "Test");
                            }
                            _ => panic!("Expected text node within <span>"),
                        }
                    }
                    _ => panic!("Expected an element node <span> as child of <div>"),
                }
            }
            _ => panic!("Expected a single element node <div>"),
        }
        Ok(())
    }

    #[test]
    fn test_parse_comment() -> Result<(), ParserError> {
        let input = "<!-- A comment --><div>Content</div>";
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 2);

        match &doc.root_nodes[0] {
            HtmlNode::Comment(comment_text) => {
                assert_eq!(comment_text, " A comment ");
            }
            _ => panic!("First node should be a comment"),
        }

        match &doc.root_nodes[1] {
            HtmlNode::Element(element) => {
                assert_eq!(element.tag_name, "div");
                assert_eq!(element.children.len(), 1);
                match &element.children[0] {
                    HtmlNode::Text(text) => {
                        assert_eq!(text, "Content");
                    }
                    _ => panic!("Expected text node in <div>"),
                }
            }
            _ => panic!("Expected an element node as second child"),
        }

        Ok(())
    }

    #[test]
    fn test_parse_multiple_siblings() -> Result<(), ParserError> {
        let input = "<p>One</p><p>Two</p><p>Three</p>";
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 3);

        let text_values: Vec<String> = doc.root_nodes.iter().map(|node| {
            if let HtmlNode::Element(e) = node {
                if let Some(HtmlNode::Text(t)) = e.children.get(0) {
                    return t.clone();
                }
            }
            panic!("Expected a <p> element with text content")
        }).collect();

        assert_eq!(text_values, vec!["One", "Two", "Three"]);
        Ok(())
    }

    #[test]
    fn test_parse_attributes() -> Result<(), ParserError> {
        let input = r#"<div id="main" class="container"></div>"#;
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 1);

        match &doc.root_nodes[0] {
            HtmlNode::Element(element) => {
                assert_eq!(element.tag_name, "div");
                // Check attributes
                let mut attr_map = std::collections::HashMap::new();
                for (key, value) in &element.attributes {
                    attr_map.insert(key.clone(), value.clone());
                }
                assert_eq!(attr_map.get("id").map(|s| s.as_str()), Some("main"));
                assert_eq!(attr_map.get("class").map(|s| s.as_str()), Some("container"));
            }
            _ => panic!("Expected a single element node"),
        }

        Ok(())
    }

    #[test]
    fn test_unclosed_element() {
        let input = "<div>";
        let result = parse_html(input);
        // Depending on how you handle errors, this may be Ok or Err:
        // If your parser stops at EOF without matching </div>, it could succeed or return an error
        // Here we assume it's an error:
        assert!(result.is_err(), "Parser should error on unclosed element <div>");
    }

    #[test]
    fn test_with_simple_dom() {
        let input = r#"
<HTML><HEAD><meta http-equiv="content-type" content="text/html;charset=utf-8">
<TITLE>301 Moved</TITLE></HEAD><BODY>
<H1>301 Moved</H1>
The document has moved
<A HREF="http://www.google.fr/">here</A>.
</BODY></HTML>"#;
        let doc = parse_html(input).unwrap();
        assert_eq!(doc.root_nodes.len(), 1);
        match &doc.root_nodes[0] {
            HtmlNode::Element(element) => {
                assert_eq!(element.tag_name, "HTML");
                assert_eq!(element.children.len(), 2);
            }
            _ => panic!("Expected a single element node"),
        }
    }

    #[test]
    fn test_button_value_with_spaces() -> Result<(), ParserError> {
        let input = r#"<button value="Et un bouton"></button>"#;
        let doc = parse_html(input)?;
        assert_eq!(doc.root_nodes.len(), 1);

        match &doc.root_nodes[0] {
            HtmlNode::Element(element) => {
                // Check the tag name
                assert_eq!(element.tag_name.to_lowercase(), "button");

                // Check that the "value" attribute includes the entire string, with spaces
                assert_eq!(element.get_attribute("value"), Some("Et un bouton"));
            }
            _ => panic!("Expected a <button> element node"),
        }

        Ok(())
    }
}
