//! This module defines the HTTP `Response` struct to represent parsed HTTP responses.

/// Represents a parsed HTTP response.
#[derive(Debug)]
pub struct Response {
    /// The HTTP status code (e.g., 200, 404, 500).
    pub status_code: u16,
    /// The reason phrase (e.g., "OK" for 200).
    pub reason_phrase: String,
    /// A collection of headers as key-value pairs.
    pub headers: Vec<(String, String)>,
    /// The response body as raw bytes.
    pub body: Vec<u8>,
}

impl Response {
    /// Retrieves a header value (if it exists) by key.
    ///
    /// # Arguments
    /// * `key` - The header name (case-insensitive).
    ///
    /// # Returns
    /// An `Option<String>` which is `Some(value)` if found, or `None` otherwise.
    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.clone())
    }
}
