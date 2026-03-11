use super::method::HttpMethod;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Request {
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
    pub body: Option<String>,
}
