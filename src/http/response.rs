use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration: Duration,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}
