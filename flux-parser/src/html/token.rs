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
/// Now updated with a more robust way of parsing attributes
/// so that spaces inside quotes are not lost.
pub fn tokenize(input: &str) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Skip leading whitespace (outside of tags)
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Check for comment: `<!-- ... -->`
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

        // Check for a start/end tag `< ... >`
        if chars[i] == '<' {
            // Find the '>' that closes this tag
            if let Some(end) = find_char(&chars, i + 1, '>') {
                // Everything between `<` and `>` is our tag body
                let tag_body: String = chars[i + 1..end].iter().collect();
                i = end + 1; // move index past `>`

                // Check if it's an end tag: `</...>`
                if tag_body.starts_with('/') {
                    // e.g. "</div>"
                    let tag_name = tag_body[1..].trim().to_string();
                    tokens.push(Token::EndTag(tag_name));
                } else {
                    // It's a start tag. Example: `div class="something"`
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

/// Helper function to find a single character `c` in `chars` starting at `start`.
fn find_char(chars: &[char], start: usize, c: char) -> Option<usize> {
    for (index, &ch) in chars.iter().enumerate().skip(start) {
        if ch == c {
            return Some(index);
        }
    }
    None
}

/// Helper function to find a sequence of characters `seq` in `chars` starting at `start`.
/// Returns the index of the sequence start if found.
fn find_sequence(chars: &[char], start: usize, seq: &[char]) -> Option<usize> {
    let seq_len = seq.len();
    'outer: for i in start..=chars.len().saturating_sub(seq_len) {
        for j in 0..seq_len {
            if chars[i + j] != seq[j] {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}

/// Parse a full tag body like:  `div class="hello" id="main"`
/// and return `(tag_name, Vec<(attribute, value)>)`.
/// This version does a small character-by-character parse
/// so that spaces inside quoted values are preserved.
fn parse_tag_and_attributes(tag_body: String) -> Result<(String, Vec<(String, String)>), ParserError> {
    let mut chars = tag_body.trim().chars().peekable();

    // 1) Parse the tag name
    let tag_name = parse_tag_name(&mut chars)?;

    // 2) Parse all attributes until the end of the tag body
    let attributes = parse_attributes(&mut chars)?;

    Ok((tag_name, attributes))
}

/// Parse the first "word" from `chars` as the tag name.
fn parse_tag_name<I: Iterator<Item=char>>(chars: &mut std::iter::Peekable<I>) -> Result<String, ParserError> {
    skip_whitespace(chars);

    let mut tag_name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            break;
        }
        tag_name.push(c);
        chars.next();
    }

    if tag_name.is_empty() {
        return Err(ParserError::Other("Empty tag body".into()));
    }

    Ok(tag_name)
}

/// Parse all attributes from the current cursor until we reach the end of the tag
/// (i.e., `'>'` or we've consumed all characters).
fn parse_attributes<I: Iterator<Item=char>>(chars: &mut std::iter::Peekable<I>) -> Result<Vec<(String, String)>, ParserError> {
    let mut attributes = Vec::new();

    loop {
        skip_whitespace(chars);

        // If we've exhausted the tag body, break
        if chars.peek().is_none() {
            // No more chars
            break;
        }

        // If we see something like `>` or `/`, we've reached the tag's end
        match chars.peek() {
            Some('>') | Some('/') => {
                // Just break; main tokenizer logic will handle the rest
                break;
            }
            _ => {}
        }

        // Parse attribute name
        let name = match parse_attribute_name(chars) {
            Ok(n) => n,
            Err(_) => {
                // If we fail here, it could be that there's trailing slash or something
                break;
            }
        };

        skip_whitespace(chars);

        // Parse optional `= value`
        let mut value = String::new();
        if let Some('=') = chars.peek() {
            // Consume '='
            chars.next();
            skip_whitespace(chars);

            // Parse either a quoted or unquoted value
            value = parse_attribute_value(chars)?;
        }

        attributes.push((name, value));
    }

    Ok(attributes)
}

/// Parse a single attribute name, stopping at whitespace, `=`, `>` or `/`.
fn parse_attribute_name<I: Iterator<Item=char>>(chars: &mut std::iter::Peekable<I>) -> Result<String, ParserError> {
    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '=' || c == '>' || c == '/' {
            break;
        }
        name.push(c);
        chars.next();
    }

    if name.is_empty() {
        return Err(ParserError::Other("Missing attribute name".into()));
    }

    Ok(name)
}

/// Parse a single attribute value, which may be quoted or unquoted.
fn parse_attribute_value<I: Iterator<Item=char>>(chars: &mut std::iter::Peekable<I>) -> Result<String, ParserError> {
    let mut value = String::new();

    // If it's a quoted value
    if let Some(&quote_char) = chars.peek() {
        if quote_char == '"' || quote_char == '\'' {
            // Consume the quote
            chars.next();

            // Read until the matching quote
            while let Some(&c) = chars.peek() {
                chars.next();
                if c == quote_char {
                    // End of quoted value
                    break;
                }
                value.push(c);
            }
            return Ok(value);
        }
    }

    // Otherwise, parse unquoted value until whitespace or tag end
    while let Some(&c) = chars.peek() {
        // Stop on whitespace, or possible tag delimiter
        if c.is_whitespace() || c == '>' || c == '/' {
            break;
        }
        value.push(c);
        chars.next();
    }

    Ok(value)
}

/// Utility: consume and discard any consecutive whitespace from the current cursor
fn skip_whitespace<I: Iterator<Item=char>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}
