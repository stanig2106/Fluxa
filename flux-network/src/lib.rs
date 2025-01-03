//! This is the main entry file for the flux-network crate.
//! It re-exports the public API for external usage.

mod errors;
mod client;
mod request;
mod response;
mod http_parser;
mod url;
mod settings;

use crate::request::Method;
pub use client::HttpClient;
pub use errors::NetworkError;
pub use request::Request;
pub use response::Response;

/// Public method to fetch data from a given URL.
///
/// # Arguments
/// * `url` - The URL to fetch (e.g., "http://example.com").
///
/// # Returns
/// A `Result<Response, NetworkError>` which represents either a valid HTTP response or an error.
///
/// # Example
/// ```rust,no_run
/// use futures::executor::block_on;
/// match block_on(flux_network::fetch("http://example.com")) {
///     Ok(response) => {
///         println!("Status code: {:?}", response);
///     }
///     Err(err) => {
///         eprintln!("Error: {:#?}", err);
///     }
/// }
///
/// ```
pub async fn fetch(url: &str) -> Result<Response, NetworkError> {
    // Parse l’URL
    let parsed_url = url::parse_url(url)?;

    if parsed_url.scheme == "fluxa" {
        return settings::fetch_settings(parsed_url);
    }

    let mut client = HttpClient::new();

    // Connexion au serveur
    client.connect(&parsed_url.host, parsed_url.port)?;

    // Prépare la requête HTTP
    let mut request = Request::new(Method::GET, &parsed_url.path);
    request.add_header("Host", &parsed_url.host);
    request.add_header("User-Agent", "FluxNetwork/0.1");

    // Envoie la requête
    client.send_request(&request)?;

    // Récupère la réponse brute
    let raw_response_data = client.receive_raw_response()?;

    // Parse la réponse dans une structure `Response`
    let response = client.parse_response(&raw_response_data)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        // This is an integration test that requires an active internet connection.
        // It sends a real HTTP request to example.com and checks the response.
        let future = fetch("http://example.com");
        let response = futures::executor::block_on(future).unwrap();
        assert_eq!(response.status_code, 200);
    }
}
