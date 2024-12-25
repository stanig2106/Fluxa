//! This module defines the HTTP `Response` struct to represent parsed HTTP responses.

/// Represents a parsed HTTP response.
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
    /// Creates a new `Response` with default/empty values.
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Retrieves a header value (if it exists) by key.
    ///
    /// # Arguments
    /// * `key` - The header name (case-insensitive).
    ///
    /// # Returns
    /// An `Option<String>` which is `Some(value)` if found, or `None` otherwise.
    pub fn get_header(&self, key: &str) -> Option<String> {
        unimplemented!()
    }
}
