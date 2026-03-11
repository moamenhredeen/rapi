use std::str::FromStr;
use std::time::Instant;

use reqwest::header::HeaderValue;

use super::request::Request;
use super::response::Response;

pub async fn execute(request: Request) -> Result<Response, String> {
    let start = Instant::now();
    let client = reqwest::Client::new();

    let url = reqwest::Url::from_str(&request.url)
        .map_err(|e| format!("Invalid URL: {e}"))?;

    let method: reqwest::Method = request.method.into();
    let mut req = reqwest::Request::new(method, url);

    for (key, value) in &request.headers {
        let header_value = HeaderValue::from_str(value)
            .map_err(|e| format!("Invalid header value for '{key}': {e}"))?;
        req.headers_mut().append(
            reqwest::header::HeaderName::from_str(key)
                .map_err(|e| format!("Invalid header name '{key}': {e}"))?,
            header_value,
        );
    }

    if !request.headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("content-type")) {
        req.headers_mut().append(
            "Content-Type",
            HeaderValue::from_static("application/json"),
        );
    }

    if let Some(body) = request.body {
        *req.body_mut() = Some(body.into());
    }

    let resp = client.execute(req).await
        .map_err(|e| format!("Request failed: {e}"))?;

    let status_code = resp.status().as_u16();
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
        .collect();

    let body = resp.text().await
        .map_err(|e| format!("Failed to read response: {e}"))?;

    let duration = start.elapsed();

    Ok(Response {
        status_code,
        headers,
        body,
        duration,
    })
}
