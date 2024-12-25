//! This module defines the HTTP `Request` struct and helper methods to build requests.

/// Represents an HTTP method (GET, POST, etc.).
#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    // Add more methods as needed
}

/// Represents an HTTP request, including method, path, headers, and body.
pub struct Request {
    /// The HTTP method to use (GET, POST, etc.).
    pub method: Method,
    /// The path or URI (e.g., "/index.html" or "/").
    pub path: String,
    /// A collection of headers as key-value pairs.
    pub headers: Vec<(String, String)>,
    /// The HTTP request body (if any).
    pub body: Vec<u8>,
}

impl Request {
    /// Creates a new `Request` with the given method and path.
    /// Headers and body can be added later.
    pub fn new(method: Method, path: &str) -> Self {
        unimplemented!()
    }

    /// Adds a header (key-value pair) to the request.
    ///
    /// # Arguments
    /// * `key` - The header name (e.g. "Content-Type").
    /// * `value` - The header value (e.g. "application/json").
    pub fn add_header(&mut self, key: &str, value: &str) {
        unimplemented!()
    }

    /// Sets the request body from a slice of bytes.
    ///
    /// # Arguments
    /// * `data` - The raw data for the request body.
    pub fn set_body(&mut self, data: &[u8]) {
        unimplemented!()
    }

    /// Converts the `Request` into a valid HTTP/1.1 request string.
    ///
    /// # Returns
    /// A `String` that can be sent directly to the server over a TCP connection.
    pub fn to_http_string(&self, host: &str) -> String {
        unimplemented!()
    }
}
