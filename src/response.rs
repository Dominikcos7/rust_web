pub use crate::response::status_line::StatusLine;

use httpdate::fmt_http_date;
use std::collections::HashMap;

mod status_line;

#[derive(Debug)]
pub struct Response {
    status_line: StatusLine,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn as_bytes(self: &Self) -> Vec<u8> {
        let mut s: String = self.status_line.as_string();
        s.push_str("\r\n");

        for (header, value) in self.headers.iter() {
            s.push_str(format!("{header}: {value}\r\n").as_str());
        }

        s.push_str("\r\n");

        if self.body.is_some() {
            s.push_str(self.body.as_ref().unwrap().as_str());
        }

        s.into_bytes()
    }

    pub fn builder() -> ResponseBuilder {
        ResponseBuilder { 
            status_line: None,
            headers: None,
            body: None,
        }
    }
}

pub struct ResponseBuilder {
    status_line: Option<StatusLine>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl ResponseBuilder {
    pub fn body(mut self: Self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self: Self) -> Response {
        if let Some(h) = &self.headers {
            if !h.contains_key(&"Date".to_string()) {
                panic!("Response should contain Date header.");
            }

            if !h.contains_key(&"Content-Length".to_string()) {
                panic!("Response should contain Content-Length header.");
            }
        }

        Response {
            status_line: self.status_line.expect("Setting a status line is required."),
            headers: self.headers.expect("Setting at least one header is required."),
            body: self.body,
        }
    }

    pub fn header(mut self: Self, name: &str, value: &str) -> Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }

        self.headers.as_mut().unwrap().insert(name.to_string(), value.to_string());

        self
    }

    pub fn status_line(mut self: Self, status_line: StatusLine) -> Self {
        self.status_line = Some(status_line);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    fn get_default_response_builder() -> ResponseBuilder {
        let builder = Response::builder()
            .status_line(StatusLine::from_str("HTTP/1.1 200 OK"))
            .header("Date", &fmt_http_date(SystemTime::now()))
            .header("Content-Type", "text/html");

        builder
    }

    #[test]
    #[should_panic]
    fn builder_should_not_build_if_status_line_is_none() {
        let builder = Response::builder()
            .header("Date", &fmt_http_date(SystemTime::now()))
            .header("Content-Type", "text/html");
        builder.build();
    }

    #[test]
    #[should_panic]
    fn builder_should_not_build_if_headers_is_none() {
        let builder = Response::builder()
            .status_line(StatusLine::from_str("HTTP/1.1 200 OK"));
        builder.build();
    }

    #[test]
    #[should_panic]
    fn builder_should_not_build_if_date_header_is_missing() {
        let builder = Response::builder()
            .status_line(StatusLine::from_str("HTTP/1.1 200 OK"))
            .header("Content-Type", "text/html");
        builder.build();
    }

    #[test]
    #[should_panic]
    fn builder_should_not_build_if_content_type_header_is_missing() {
        let builder = Response::builder()
            .status_line(StatusLine::from_str("HTTP/1.1 200 OK"))
            .header("Date", &fmt_http_date(SystemTime::now()));
        builder.build();
    }
}