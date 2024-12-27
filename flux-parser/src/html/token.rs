/// token.rs

use crate::error::ParserError;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// A start tag: `<div ...>`
    StartTag(String, Vec<(String, String)>),

    /// An end tag: `</div>`
    EndTag(String),

    /// Text content between tags
    Text(String),

    /// A comment: `<!-- comment -->`
    Comment(String),

    /// End of file / input
    Eof,
}

/// A very naive tokenizer that splits the input into tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Check for comment
        if i + 3 < chars.len() && &chars[i..i + 4] == ['<', '!', '-', '-'] {
            // Try to find `-->`
            if let Some(end) = find_sequence(&chars, i + 4, &['-', '-', '>']) {
                let comment_text: String = chars[i + 4..end].iter().collect();
                tokens.push(Token::Comment(comment_text));
                i = end + 3; // skip past `-->`
                continue;
            } else {
                return Err(ParserError::Other("Unclosed comment".into()));
            }
        }

        // Check for start tag or end tag
        if chars[i] == '<' {
            // Find the closing '>'
            if let Some(end) = find_char(&chars, i + 1, '>') {
                let tag_body: String = chars[i + 1..end].iter().collect();
                i = end + 1; // move past '>'

                // Check if it's an end tag
                if tag_body.starts_with('/') {
                    // e.g. </div>
                    let tag_name = tag_body[1..].trim().to_string();
                    tokens.push(Token::EndTag(tag_name));
                } else {
                    // e.g. <div class="something">
                    // We need to parse out attributes
                    let (tag_name, attributes) = parse_tag_and_attributes(tag_body)?;
                    tokens.push(Token::StartTag(tag_name, attributes));
                }
                continue;
            } else {
                return Err(ParserError::Other("Tag not closed".into()));
            }
        }

        // Otherwise, it's text until the next '<' or EOF
        let start_text = i;
        while i < chars.len() && chars[i] != '<' {
            i += 1;
        }
        let text_content: String = chars[start_text..i].iter().collect();
        tokens.push(Token::Text(text_content));
    }

    // Finally, add EOF token
    tokens.push(Token::Eof);
    Ok(tokens)
}

/// Helper function to find a single character in `chars` starting at `start`.
fn find_char(chars: &[char], start: usize, c: char) -> Option<usize> {
    for (index, &ch) in chars.iter().enumerate().skip(start) {
        if ch == c {
            return Some(index);
        }
    }
    None
}

/// Helper function to find a sequence of characters in `chars` starting at `start`.
fn find_sequence(chars: &[char], start: usize, seq: &[char]) -> Option<usize> {
    // We want to find the start index of a contiguous sequence `seq`.
    // e.g. `seq = ['-', '-', '>']`
    let seq_len = seq.len();
    'outer: for i in start..=chars.len() - seq_len {
        for j in 0..seq_len {
            if chars[i + j] != seq[j] {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}

/// Given a string containing something like `div class="hello" id="main"`,
/// extract the `tag_name` and a vector of `(attribute, value)` pairs.
/// For a real parser, you'd want robust handling of single quotes, no quotes, etc.
fn parse_tag_and_attributes(s: String) -> Result<(String, Vec<(String, String)>), ParserError> {
    let mut parts = s.trim().split_whitespace();
    let tag_name = parts
        .next()
        .ok_or_else(|| ParserError::Other("Empty tag body".into()))?
        .to_string();

    let mut attributes = Vec::new();
    for part in parts {
        // e.g. part could be `class="hello"`
        if let Some(eq_index) = part.find('=') {
            let attr_name = &part[..eq_index];
            let attr_value = &part[eq_index + 1..];
            let attr_value_clean = attr_value.trim_matches('"').to_string();
            attributes.push((attr_name.to_string(), attr_value_clean));
        } else {
            // Might be a boolean attribute or something else
            attributes.push((part.to_string(), String::new()));
        }
    }

    Ok((tag_name, attributes))
}
