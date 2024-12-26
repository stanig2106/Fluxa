//! This module defines the HTTP `Request` struct and helper methods to build requests.

use std::fmt;

/// Represents an HTTP method (GET, POST, etc.).
#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
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
        Self {
            method,
            path: path.to_owned(),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    /// Adds a header (key-value pair) to the request.
    ///
    /// # Arguments
    /// * `key` - The header name (e.g. "Content-Type").
    /// * `value` - The header value (e.g. "application/json").
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.to_string(), value.to_string()));
    }

    /// Sets the request body from a slice of bytes.
    ///
    /// # Arguments
    /// * `data` - The raw data for the request body.
    pub fn set_body(&mut self, data: &[u8]) {
        self.body = data.to_vec();
    }

    /// Converts the `Request` into a valid HTTP/1.1 request string.
    ///
    /// # Returns
    /// A `String` that can be sent directly to the server over a TCP connection.
    pub fn to_http_string(&self) -> String {
        // Convert our Method enum to the corresponding HTTP verb text
        let method_str = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
        };

        // Start building the request line: "GET /path HTTP/1.1\r\n"
        let mut request = format!("{} {} HTTP/1.1\r\n", method_str, self.path);

        // Add any additional headers
        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }

        // If there is a body, indicate Content-Length
        if !self.body.is_empty() {
            request.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        }

        // A blank line separates the header section from the body
        request.push_str("\r\n");

        // If there's a body, append it (assuming it's UTF-8 text for now #todo !)
        if !self.body.is_empty() {
            let body_str = String::from_utf8_lossy(&self.body);
            request.push_str(&body_str);
        }

        request
    }

}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // display the http_string
        write!(f, "{}", self.to_http_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_request() {
        let request = Request::new(Method::GET, "/test");

        // Verify method
        matches!(request.method, Method::GET);
        // Verify path
        assert_eq!(request.path, "/test");
        // Headers and body should be empty initially
        assert!(request.headers.is_empty());
        assert!(request.body.is_empty());
    }

    #[test]
    fn test_add_header() {
        let mut request = Request::new(Method::GET, "/");
        request.add_header("Content-Type", "application/json");

        assert_eq!(request.headers.len(), 1);
        assert_eq!(
            request.headers[0],
            ("Content-Type".to_string(), "application/json".to_string())
        );
    }

    #[test]
    fn test_set_body() {
        let mut request = Request::new(Method::POST, "/upload");
        let data = b"Hello, world!";
        request.set_body(data);

        assert_eq!(request.body, data.to_vec());
    }

    #[test]
    fn test_to_http_string_no_body() {
        let mut request = Request::new(Method::GET, "/test");
        request.add_header("User-Agent", "MyTestAgent/1.0");

        let http_str = request.to_http_string();

        // Check the request line
        assert!(http_str.starts_with("GET /test HTTP/1.1\r\n"));
        // Check our custom header
        assert!(http_str.contains("User-Agent: MyTestAgent/1.0\r\n"));
        // Body should be empty
        assert!(!http_str.contains("Content-Length"));
        // Should end with a blank line (no Body content)
        assert!(http_str.ends_with("\r\n\r\n"));
    }

    #[test]
    fn test_to_http_string_with_body() {
        let mut request = Request::new(Method::POST, "/submit");
        request.add_header("Content-Type", "application/json");
        request.add_header("Accept", "application/json");

        let body_data = br#"{"key":"value"}"#;
        request.set_body(body_data);

        let http_str = request.to_http_string();

        // Request line
        assert!(http_str.starts_with("POST /submit HTTP/1.1\r\n"));
        // Custom headers
        assert!(http_str.contains("Content-Type: application/json\r\n"));
        assert!(http_str.contains("Accept: application/json\r\n"));
        // Correct Content-Length
        let expected_len = body_data.len();
        assert!(http_str.contains(&format!("Content-Length: {expected_len}\r\n")));
        // Body content at the end
        assert!(http_str.ends_with("{\"key\":\"value\"}"));
    }
}
