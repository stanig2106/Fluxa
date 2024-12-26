use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::{NetworkError, Request, Response};
use crate::http_parser::parse_http_response;

/// A basic HTTP client that can send requests and receive responses over TCP.
/// It manages the connection, sending HTTP requests, and reading raw HTTP responses.
pub struct HttpClient {
    // We store a `TcpStream` for the active connection.
    // It is wrapped in an Option because the stream may not be initialized
    // until `connect` is called.
    stream: Option<TcpStream>,
}

impl HttpClient {
    /// Creates a new `HttpClient` instance with default settings.
    pub fn new() -> Self {
        HttpClient { stream: None }
    }

    /// Opens a TCP connection to the given host and port.
    ///
    /// # Arguments
    /// * `host` - Host name or IP address (e.g., "example.com").
    /// * `port` - TCP port number (e.g., 80 for HTTP).
    ///
    /// # Returns
    /// A `Result` indicating success or an error.
    pub fn connect(&mut self, host: &str, port: u16) -> Result<(), NetworkError> {
        // Convert (host, port) to a valid socket address string, e.g. "example.com:80".
        let socket_address = format!("{}:{}", host, port);

        // Attempt to establish the TCP connection.
        // Any I/O error is mapped to a `NetworkError::ConnectionFailed` (or whatever error variant you have).
        let stream = TcpStream::connect(socket_address)
            .map_err(|_| NetworkError::ConnectionError("ConnectionFailed".to_string()))?;

        // Store the connected stream for further use.
        self.stream = Some(stream);
        Ok(())
    }

    /// Sends an HTTP request over the established connection.
    ///
    /// # Arguments
    /// * `request` - The `Request` object containing HTTP method, headers, and body.
    ///
    /// # Returns
    /// A `Result` indicating success or an error.
    pub fn send_request(&mut self, request: &Request) -> Result<(), NetworkError> {
        // Ensure we have a valid connection before attempting to write.
        let stream = match self.stream.as_mut() {
            Some(s) => s,
            None => return Err(NetworkError::ConnectionError("ConnectionNotEstablished".to_string())),
        };

        // Convert the `Request` into its raw HTTP form (e.g., "GET /path HTTP/1.1\r\nHost: ...\r\n\r\n").
        let http_data = request.to_http_string(); // If you have a method to build the HTTP request.

        // Write the request headers/lines to the TCP stream.
        stream
            .write_all(http_data.as_bytes())
            .map_err(|_| NetworkError::IoError("WriteFailed".to_string()))?;


        // Flush ensures everything is sent out promptly.
        stream.flush().map_err(|_| NetworkError::IoError("FlushFailed".to_string()))?;
        Ok(())
    }

    /// Receives the raw HTTP response from the server in a more robust way,
    /// handling both `Content-Length` and `Transfer-Encoding: chunked`.
    ///
    /// # Returns
    /// A `Result<Vec<u8>, NetworkError>` representing the raw bytes (headers + body).
    pub fn receive_raw_response(&mut self) -> Result<Vec<u8>, NetworkError> {
        // Ensure we have a valid connection before attempting to read.
        let stream = match self.stream.as_mut() {
            Some(s) => s,
            None => {
                return Err(NetworkError::ConnectionError(
                    "ConnectionNotEstablished".to_string(),
                ))
            }
        };

        // We'll read the data using a buffered reader, but since we want to return
        // the entire raw response, we also need to accumulate the bytes we read.
        let mut buffer_reader = BufReader::new(stream);

        // Step 1: Read until we have the complete HTTP headers.
        //         We'll accumulate bytes into `raw_headers`.
        let mut raw_headers = Vec::new();
        loop {
            // Read byte by byte
            let mut byte = [0u8; 1];
            // If we fail to read, return an error
            if buffer_reader.read_exact(&mut byte).is_err() {
                return Err(NetworkError::IoError("ReadFailed".to_string()));
            }

            raw_headers.push(byte[0]);

            // Check if we've reached the end of headers: "\r\n\r\n" or "\n\n".
            if raw_headers.ends_with(b"\r\n\r\n") || raw_headers.ends_with(b"\n\n") {
                break;
            }
        }

        // Convert the header bytes to a string for parsing.
        let header_str =
            String::from_utf8_lossy(&raw_headers).to_string();

        // Step 2: Parse out the headers to check for `Content-Length` or `Transfer-Encoding`.
        //         We only demonstrate a minimal approach here.
        let mut content_length: Option<usize> = None;
        let mut is_chunked = false;

        for line in header_str.lines() {
            let lower = line.to_lowercase();
            if lower.starts_with("content-length:") {
                if let Some(cl) = line.split(':').nth(1) {
                    // Trim whitespace and parse the number
                    let cl = cl.trim();
                    if let Ok(parsed) = cl.parse::<usize>() {
                        content_length = Some(parsed);
                    }
                }
            }
            if lower.starts_with("transfer-encoding:") {
                if line.to_lowercase().contains("chunked") {
                    is_chunked = true;
                }
            }
        }

        // We'll place the raw response here (headers + body).
        let mut full_response = raw_headers.clone();

        // Step 3: Depending on content length or chunked encoding, read the body accordingly.

        if is_chunked {
            // Step 3a: Chunked encoding
            let mut body = Vec::new();

            loop {
                // 3a-i: Read one line to get the chunk size in hex
                let mut size_line = String::new();
                if buffer_reader.read_line(&mut size_line).is_err() {
                    return Err(NetworkError::IoError("ChunkSizeReadFailed".to_string()));
                }

                // Append chunk size line to the "full_response" so we maintain the raw data
                full_response.extend_from_slice(size_line.as_bytes());

                // Convert hex size to decimal
                let chunk_size = match usize::from_str_radix(size_line.trim(), 16) {
                    Ok(size) => size,
                    Err(_) => {
                        return Err(NetworkError::ParseError(
                            "InvalidChunkSize".to_string(),
                        ))
                    }
                };

                // Zero-size chunk indicates the end of the body
                if chunk_size == 0 {
                    // Read the trailing \r\n after the zero chunk
                    let mut trailing_crlf = vec![0u8; 2];
                    if buffer_reader.read_exact(&mut trailing_crlf).is_err() {
                        return Err(NetworkError::IoError(
                            "ChunkTrailingCRLFReadFailed".to_string(),
                        ));
                    }
                    full_response.extend_from_slice(&trailing_crlf);
                    break;
                }

                // 3a-ii: Read the actual chunk
                let mut chunk_data = vec![0u8; chunk_size];
                if buffer_reader.read_exact(&mut chunk_data).is_err() {
                    return Err(NetworkError::IoError("ChunkDataReadFailed".to_string()));
                }

                // 3a-iii: Read the trailing \r\n
                let mut crlf = vec![0u8; 2];
                if buffer_reader.read_exact(&mut crlf).is_err() {
                    return Err(NetworkError::IoError("ChunkCRLFReadFailed".to_string()));
                }

                // Accumulate in our body buffer
                body.extend_from_slice(&chunk_data);

                // Also accumulate in the raw `full_response`
                full_response.extend_from_slice(&chunk_data);
                full_response.extend_from_slice(&crlf);
            }
        } else if let Some(length) = content_length {
            // Step 3b: Read exactly `Content-Length` bytes
            let mut body = vec![0u8; length];
            if buffer_reader.read_exact(&mut body).is_err() {
                return Err(NetworkError::IoError("ReadBodyFailed".to_string()));
            }
            full_response.extend_from_slice(&body);
        } else {
            // Step 3c: No content length, not chunked -> read until closure with a 10s timeout.

            {
                // 1) Borrow just long enough to set the timeout, then drop.
                let inner_stream = buffer_reader.get_mut();
                inner_stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .map_err(|_| NetworkError::IoError("SetReadTimeoutFailed".to_string()))?;
            } // `inner_stream` goes out of scope here, so the mutable borrow ends.

            // Now we can safely call `buffer_reader.read_to_end(...)`.
            let mut body = Vec::new();
            if buffer_reader.read_to_end(&mut body).is_err() {
                return Err(NetworkError::IoError("ReadBodyTimeoutOrFailed".to_string()));
            }
            full_response.extend_from_slice(&body);

            {
                // 2) (Optional) Borrow again if you want to reset the timeout.
                let inner_stream = buffer_reader.get_mut();
                inner_stream
                    .set_read_timeout(None)
                    .map_err(|_| NetworkError::IoError("ResetReadTimeoutFailed".to_string()))?;
            }
        }

        // Step 4: Return the full response (headers + body).
        Ok(full_response)
    }

    /// Parses the raw response bytes and returns a `Response`.
    ///
    /// # Arguments
    /// * `raw_data` - The raw HTTP response data from `receive_raw_response`.
    ///
    /// # Returns
    /// A `Result<Response, NetworkError>` representing a parsed HTTP response or an error.
    pub fn parse_response(&self, raw_data: &[u8]) -> Result<Response, NetworkError> {
        // Use the provided `parse_http_response` function to parse the raw data.
        parse_http_response(raw_data)
    }
}
