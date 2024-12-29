use crate::url::Url;
use crate::{NetworkError, Response};

pub(crate) fn fetch_settings(url: Url) -> Result<Response, NetworkError> {
    let host = url.host;

    let content = match host.as_str() {
        "hello" => include_str!("hello.html"),
        _ => return Err(NetworkError::NotFound),
    };

    Ok(Response {
        status_code: 200,
        reason_phrase: "OK".to_string(),
        headers: vec![],
        body: content.as_bytes().to_vec(),
    })
}
