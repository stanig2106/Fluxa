//! This is the main entry file for the flux-network crate.
//! It re-exports the public API for external usage.

mod errors;
mod client;
mod request;
mod response;
mod http_parser;
mod url;

pub use client::HttpClient;
pub use errors::NetworkError;
pub use request::Request;
pub use response::Response;
use crate::request::Method;

/// Public method to fetch data from a given URL.
///
/// # Arguments
/// * `url` - The URL to fetch (e.g., "http://example.com").
///
/// # Returns
/// A `Result<Response, NetworkError>` which represents either a valid HTTP response or an error.
///
/// # Example
/// ```no_run
/// match flux_network::fetch("http://example.com") {
///     Ok(response) => {
///         println!("Status: {}", response.status_code);
///     }
///     Err(e) => {
///         eprintln!("Error: {:?}", e);
///     }
/// }
/// ```

pub fn fetch(url: &str) -> Result<Response, NetworkError> {
    // Parse the given URL into its components (scheme, host, port, path, etc.).
    let parsed_url = url::parse_url(url)?;

    // Create a new instance of the HTTP client.
    let mut client = HttpClient::new();

    // Connect to the server using the parsed host and port.
    // If no port is specified, default to 80 for HTTP.
    client.connect(&parsed_url.host, parsed_url.port)?;

    // Construct a new request (for example, use the GET method and the parsed path).
    // You may want to handle different HTTP methods in the future.
    let mut request = Request::new(Method::GET, &parsed_url.path);

    // Add any required headers (here, we set a Host header and a basic User-Agent).
    request.add_header("Host", &parsed_url.host);
    request.add_header("User-Agent", "FluxNetwork/0.1");

    println!("{}", request);

    // Send the request through the client.
    client.send_request(&request)?;

    // Retrieve the raw response data from the server (this is just a placeholder
    // method name and needs to be implemented in HttpClient).
    let raw_response_data = client.receive_raw_response()?;

    // Convert the raw response data into a structured Response.
    let response = client.parse_response(&raw_response_data)?;

    // Return the Response or an error if something went wrong.
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        // This is an integration test that requires an active internet connection.
        // It sends a real HTTP request to example.com and checks the response.
        let response = fetch("http://example.com").unwrap();
        assert_eq!(response.status_code, 200);

    }
}
