//! A structure representing a parsed URL.

pub struct Url {
    /// The full URL string (e.g.,
    ///   http://username:password@exemple.com:8080/path/to/resource?key=value#section
    /// ).

    /// The scheme of the URL (e.g., "http", "https").
    pub scheme: String,
    /// The hostname of the URL (e.g., "example.com").
    pub hostname: String,
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

pub fn parse_url(url: &str) -> Result<Url, String> {
    // Vérifier si l’URL contient un schéma (ex. "http://", "https://")
    // On cherche le "://" pour séparer le schéma du reste de l’URL.
    let scheme_end = match url.find("://") {
        Some(pos) => pos,
        None => return Err("Invalid URL: missing or malformed scheme".to_string()),
    };
    let scheme = url[..scheme_end].to_string();

    // Récupération de la partie qui suit le schéma, ex. "username:password@exemple.com:8080/..."
    let remainder = &url[scheme_end + 3..];

    // On va maintenant vérifier la présence éventuelle de username/password,
    // localisés avant le '@' (s’il existe).
    let mut username = None;
    let mut password = None;
    let auth_split: Vec<&str> = remainder.split('@').collect();

    // S’il y a un '@', on considère la première partie comme les identifiants (username[:password]),
    // et la deuxième partie comme le "host:port/path?query#fragment".
    let (after_auth, _rest) = if auth_split.len() == 2 {
        let credentials = auth_split[0];
        let r = auth_split[1];

        // Vérifier s’il existe un mot de passe.
        // Format : "username:password"
        if let Some(colon_pos) = credentials.find(':') {
            username = Some(credentials[..colon_pos].to_string());
            password = Some(credentials[colon_pos + 1..].to_string());
        } else {
            username = Some(credentials.to_string());
        }
        (r, true)
    } else {
        // Pas de partie "@" → pas d’identifiants
        (remainder, false)
    };

    // Séparer la partie host:port du reste (path, query, fragment).
    // On cherche d’abord le premier '/' pour séparer le chemin
    // (on gérera ensuite query et fragment séparément).
    let path_start = after_auth.find('/').unwrap_or(after_auth.len());
    let (host_port_part, path_part) = after_auth.split_at(path_start);

    // Extraire le host et le port éventuel (ex. "exemple.com:8080").
    let mut hostname = String::new();
    let mut port: u16 = default_port(&scheme);

    if let Some(colon_pos) = host_port_part.find(':') {
        // On a un port
        hostname = host_port_part[..colon_pos].to_string();
        let port_str = &host_port_part[colon_pos + 1..];

        // Convertir le port en u16
        match port_str.parse::<u16>() {
            Ok(p) => port = p,
            Err(_) => return Err("Invalid port".to_string()),
        }
    } else {
        // Pas de port → tout est dans le hostname
        if !host_port_part.is_empty() {
            hostname = host_port_part.to_string();
        }
    }

    // Gérer le chemin, la query, et le fragment.
    // Exemple : /path/to/resource?key=value#section
    let mut path = String::new();
    let mut query = String::new();
    let mut fragment = String::new();

    if !path_part.is_empty() {
        // path_part commence par '/', on l’enlève pour faciliter la suite
        let path_part = &path_part[1..];

        // 1) Chercher d'abord le '#'.
        let hash_pos = path_part.find('#').unwrap_or(path_part.len());
        let (before_hash, after_hash) = path_part.split_at(hash_pos);

        // Si on a trouvé un '#', la suite after_hash commence par '#'
        if hash_pos < path_part.len() {
            fragment = after_hash[1..].to_string();
        }

        // 2) Dans la partie avant le '#', on cherche un éventuel '?'
        let query_start = before_hash.find('?').unwrap_or(before_hash.len());
        let (path_str, query_part) = before_hash.split_at(query_start);

        // Si on a trouvé un '?', la suite query_part commence par '?'
        if query_start < before_hash.len() {
            query = query_part[1..].to_string();
        }

        // 3) Le reste est le chemin
        path = path_str.to_string();
    }

    Ok(Url {
        scheme,
        hostname,
        port,
        path: format!("/{}", path),
        query,
        fragment,
        username,
        password,
    })
}

fn default_port(scheme: &String) -> u16 {
    match scheme.to_lowercase().as_str() {
        "http" => 80,
        "https" => 443,
        "ftp" => 21,
        _ => 80,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_basic() {
        let url_str = "http://example.com";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.hostname, "example.com");
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
        assert_eq!(parsed.hostname, "example.com");
        assert_eq!(parsed.port, 8080);
        assert_eq!(parsed.path, "/", "Path should be '/' by default");
        assert_eq!(parsed.query, "");
        assert_eq!(parsed.fragment, "");
    }

    #[test]
    fn test_parse_url_with_credentials() {
        let url_str = "http://user:pass@example.com:8080";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.hostname, "example.com");
        assert_eq!(parsed.port, 8080);
        assert_eq!(parsed.username, Some("user".to_string()));
        assert_eq!(parsed.password, Some("pass".to_string()));
    }

    #[test]
    fn test_parse_url_with_path() {
        let url_str = "https://example.com/path/to/page";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "https");
        assert_eq!(parsed.hostname, "example.com");
        assert_eq!(parsed.port, 80, "Default port is 80 (change if needed)");
        assert_eq!(parsed.path, "/path/to/page");
    }

    #[test]
    fn test_parse_url_with_query() {
        let url_str = "https://example.com/search?key=value&foo=bar";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "https");
        assert_eq!(parsed.hostname, "example.com");
        assert_eq!(parsed.path, "/search");
        assert_eq!(parsed.query, "key=value&foo=bar");
        assert_eq!(parsed.fragment, "");
    }

    #[test]
    fn test_parse_url_with_fragment() {
        let url_str = "http://example.com/page#section";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.hostname, "example.com");
        assert_eq!(parsed.path, "/page");
        assert_eq!(parsed.query, "");
        assert_eq!(parsed.fragment, "section");
    }

    #[test]
    fn test_parse_url_with_query_and_fragment() {
        let url_str = "http://example.com/path?foo=bar#title";
        let parsed = parse_url(url_str).expect("Failed to parse URL");

        assert_eq!(parsed.scheme, "http");
        assert_eq!(parsed.hostname, "example.com");
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
            assert!(err.contains("missing or malformed scheme"));
        }
    }

    #[test]
    fn test_parse_url_invalid_port() {
        let url_str = "http://example.com:abc";
        let result = parse_url(url_str);

        assert!(result.is_err(), "Parsing should fail with invalid port");
        if let Err(err) = result {
            assert!(err.contains("Invalid port"));
        }
    }
}
