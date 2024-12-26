//! This module provides helper functions to parse raw HTTP response data.

use crate::{NetworkError, Response};
use std::str;

/// Parses the raw HTTP response data into a `Response` struct.
///
/// # Arguments
/// * `raw_data` - A slice of bytes containing the complete HTTP response.
///
/// # Returns
/// A `Result<Response, NetworkError>` representing a successfully parsed response or an error.
pub fn parse_http_response(raw_data: &[u8]) -> Result<Response, NetworkError> {
    let (header_data, body_data) = split_headers_and_body(raw_data)?;

    let header_str = str::from_utf8(header_data)
        .map_err(|_| NetworkError::InvalidData("Headers are not valid UTF-8".to_string()))?;

    // The first line is the HTTP status line (e.g. "HTTP/1.1 200 OK").
    let mut lines = header_str.lines();
    let status_line = lines
        .next()
        .ok_or_else(|| NetworkError::InvalidData("No status line found".to_string()))?;

    let (status_code, reason_phrase) = parse_status_line(status_line)?;

    let header_lines: Vec<&str> = lines.collect();
    let headers = parse_headers(&header_lines);

    Ok(Response {
        status_code,
        reason_phrase,
        headers,
        body: body_data.to_vec(),
    })
}

/// Splits the raw HTTP response into headers and body.
///
/// # Arguments
/// * `raw_data` - A slice of bytes containing the HTTP response.
///
/// # Returns
/// A tuple `(header_bytes, body_bytes)` as slices of `raw_data`.
fn split_headers_and_body(raw_data: &[u8]) -> Result<(&[u8], &[u8]), NetworkError> {
    // Commonly, HTTP headers and body are separated by "\r\n\r\n".
    if let Some(pos) = raw_data.windows(4).position(|window| window == b"\r\n\r\n") {
        let header_bytes = &raw_data[..pos];
        let body_bytes = &raw_data[pos + 4..];
        Ok((header_bytes, body_bytes))
    }
    // Some servers/clients may sometimes use just "\n\n" (less common).
    else if let Some(pos) = raw_data.windows(2).position(|window| window == b"\n\n") {
        let header_bytes = &raw_data[..pos];
        let body_bytes = &raw_data[pos + 2..];
        Ok((header_bytes, body_bytes))
    } else {
        Err(NetworkError::InvalidData(
            "Could not split headers and body".to_string(),
        ))
    }
}

/// Parses the status line (e.g., "HTTP/1.1 200 OK") and returns the status code and reason phrase.
///
/// # Arguments
/// * `status_line` - A `&str` with the first line of the HTTP response.
///
/// # Returns
/// A tuple `(status_code, reason_phrase)`.
fn parse_status_line(status_line: &str) -> Result<(u16, String), NetworkError> {
    // Typically: "HTTP/1.1 200 OK"
    let mut parts = status_line.split_whitespace();

    // Skip the HTTP version (e.g., "HTTP/1.1").
    let _http_version = parts
        .next()
        .ok_or_else(|| NetworkError::InvalidData("Missing HTTP version".to_string()))?;

    // Next is the status code (e.g., 200).
    let status_code_str = parts
        .next()
        .ok_or_else(|| NetworkError::InvalidData("Missing status code".to_string()))?;
    let status_code = status_code_str
        .parse::<u16>()
        .map_err(|e| NetworkError::InvalidData(format!("Invalid status code: {}", e)))?;

    // Whatever remains is the reason phrase (e.g., "OK").
    let reason_phrase = parts.collect::<Vec<&str>>().join(" ");

    Ok((status_code, reason_phrase))
}

/// Parses the header lines into a vector of (key, value) pairs.
///
/// # Arguments
/// * `lines` - A slice of &str, each representing a header line.
///
/// # Returns
/// A vector of (key, value) pairs.
fn parse_headers(lines: &[&str]) -> Vec<(String, String)> {
    let mut headers = Vec::new();
    for line in lines {
        // Typical header line: "Content-Type: text/html"
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            headers.push((key, value));
        }
    }
    headers
}
