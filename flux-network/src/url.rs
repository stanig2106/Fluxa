//! A structure representing a parsed URL.

pub struct Url {
    /// The full URL string (e.g.,
    ///   http://username:password@exemple.com:8080/path/to/resource?key=value#section
    /// ).

    /// The scheme of the URL (e.g., "http", "https").
    pub scheme: String,
    /// The hostname of the URL (e.g., "example.com").
    pub host: String,
    /// The port number of the URL (e.g., 80, 443).
    pub port: u16,
    /// The path of the URL (e.g., "/path/to/resource").
    pub path: String,
    /// The query string of the URL (e.g., "?key=value&foo=bar").
    pub query: String,
    /// The fragment of the URL (e.g., "#section").
    pub fragment: String,
    /// The username for basic authentication.
    pub username: Option<String>,
    /// The password for basic authentication.
    pub password: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseUrlError {
    /// The URL is missing its scheme or it's malformed (e.g., no "://").
    MissingScheme,
    /// The port is not a valid u16 integer.
    InvalidPort,
}

fn default_port(scheme: &String) -> u16 {
    match scheme.to_lowercase().as_str() {
        "http" => 80,
        "https" => 443,
        "ftp" => 21,
        _ => 80,
    }
}

/// Parses a URL string and returns a `Url` struct, or an error of type `ParseUrlError`.
pub fn parse_url(url: &str) -> Result<Url, ParseUrlError> {
    // Check if the URL contains a scheme (ex: "http://", "https://")
    let scheme_end = match url.find("://") {
        Some(pos) => pos,
        None => return Err(ParseUrlError::MissingScheme),
    };
    let scheme = url[..scheme_end].to_string();

    // Retrieve everything after the scheme, e.g. "username:password@exemple.com:8080/..."
    let remainder = &url[scheme_end + 3..];

    // Look for optional username/password before '@'.
    let mut username = None;
    let mut password = None;
    let auth_split: Vec<&str> = remainder.split('@').collect();

    // If there's an '@', the first part is credentials (username[:password]),
    // and the second part is "host:port/path?query#fragment".
    let after_auth = if auth_split.len() == 2 {
        let credentials = auth_split[0];
        let host_port_path = auth_split[1];

        // Check for password: "username:password"
        if let Some(colon_pos) = credentials.find(':') {
            username = Some(credentials[..colon_pos].to_string());
            password = Some(credentials[colon_pos + 1..].to_string());
        } else {
            username = Some(credentials.to_string());
        }
        host_port_path
    } else {
        // No credentials
        remainder
    };

    // Separate "host:port" from "path?query#fragment" by looking for the first '/'.
    let path_start = after_auth.find('/').unwrap_or(after_auth.len());
    let (host_port_part, path_part) = after_auth.split_at(path_start);

    // Extract hostname and (optional) port from "host:port".
    let mut hostname = String::new();
    let mut port = default_port(&scheme);

    if let Some(colon_pos) = host_port_part.find(':') {
        hostname = host_port_part[..colon_pos].to_string();
        let port_str = &host_port_part[colon_pos + 1..];
        // Try converting port to a valid u16.
        port = match port_str.parse() {
            Ok(p) => p,
            Err(_) => return Err(ParseUrlError::InvalidPort),
        };
    } else if !host_port_part.is_empty() {
        // Host only.
        hostname = host_port_part.to_string();
    }

    // Now handle path, query, and fragment.
    let mut path = String::new();
    let mut query = String::new();
    let mut fragment = String::new();

    if !path_part.is_empty() {
        // path_part starts with '/', remove the leading slash.
        let path_part = &path_part[1..];

        // 1) Look for '#'
        let hash_pos = path_part.find('#').unwrap_or(path_part.len());
        let (before_hash, after_hash) = path_part.split_at(hash_pos);

        // If there's a '#', everything after it is the fragment.
        if hash_pos < path_part.len() {
            fragment = after_hash[1..].to_string(); // remove '#'
        }

        // 2) In the substring before '#', look for a '?'.
        let query_start = before_hash.find('?').unwrap_or(before_hash.len());
        let (path_str, query_part) = before_hash.split_at(query_start);

        // If there's a '?', everything after it is the query.
        if query_start < before_hash.len() {
            query = query_part[1..].to_string(); // remove '?'
        }

        // 3) The rest is the path
        path = path_str.to_string();
    }

    Ok(Url {
        scheme,
        host: hostname,
        port,
        path: format!("/{}", path),
        query,
        fragment,
        username,
        password,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_basic() {
        let url_str = "http://example.com";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.port, 80, "Default port should be 80 for http");
        assert_eq!(parsed.path, "/", "Path should be '/' by default");
        assert_eq!(parsed.query, "", "Query should be empty");
        assert_eq!(parsed.fragment, "", "Fragment should be empty");
        assert!(parsed.username.is_none(), "No username expected");
        assert!(parsed.password.is_none(), "No password expected");
    }

    #[test]
    fn test_default_port_https() {
        let url_str = "https://example.com";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.port, 443, "Default port should be 443 for https");
    }


    #[test]
    fn test_parse_url_with_port() {
        let url_str = "http://example.com:8080";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.port, 8080);
        assert_eq!(parsed.path, "/", "Path should be '/' by default");
        assert_eq!(parsed.query, "");
        assert_eq!(parsed.fragment, "");
    }

    #[test]
    fn test_parse_url_with_credentials() {
        let url_str = "https://user:pass@example.com:8080";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "https");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.port, 8080);
        assert_eq!(parsed.username, Some("user".to_string()));
        assert_eq!(parsed.password, Some("pass".to_string()));
    }

    #[test]
    fn test_parse_url_with_path() {
        let url_str = "https://example.com/path/to/page";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "https");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.port, 80, "Default port is 80 (change if needed)");
        assert_eq!(parsed.path, "/path/to/page");
    }

    #[test]
    fn test_parse_url_with_query() {
        let url_str = "https://example.com/search?key=value&foo=bar";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "https");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.path, "/search");
        assert_eq!(parsed.query, "key=value&foo=bar");
        assert_eq!(parsed.fragment, "");
    }

    #[test]
    fn test_parse_url_with_fragment() {
        let url_str = "http://example.com/page#section";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.path, "/page");
        assert_eq!(parsed.query, "");
        assert_eq!(parsed.fragment, "section");
    }

    #[test]
    fn test_parse_url_with_query_and_fragment() {
        let url_str = "http://example.com/path?foo=bar#title";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.host, "example.com");
        assert_eq!(parsed.path, "/path");
        assert_eq!(parsed.query, "foo=bar");
        assert_eq!(parsed.fragment, "title");
    }

    #[test]
    fn test_parse_url_missing_scheme() {
        let url_str = "example.com";
        let result = parse_url(url_str);

        assert!(result.is_err(), "Should fail without scheme");
        if let Err(err) = result {
            assert_eq!(err, ParseUrlError::MissingScheme);
        }
    }

    #[test]
    fn test_parse_url_invalid_port() {
        let url_str = "http://example.com:abc";
        let result = parse_url(url_str);

        assert!(result.is_err(), "Parsing should fail with invalid port");
        if let Err(err) = result {
            assert_eq!(err, ParseUrlError::InvalidPort);
        }
    }
}
