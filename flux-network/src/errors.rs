//! This module defines custom errors for the flux-network crate.

/// Represents all possible errors that can occur during network requests.
#[derive(Debug)]
pub enum NetworkError {
    /// An error occurred while resolving the hostname or connecting to the server.
    ConnectionError(String),
    /// An error occurred during I/O operations on the socket.
    IoError(String),
    /// An error occurred while parsing the HTTP response or request.
    ParseError(String),
    /// A generic error type for all other issues.
    Other(String),
}
