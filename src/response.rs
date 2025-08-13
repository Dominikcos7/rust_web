pub use crate::response::status_line::StatusLine;

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
        Response {
            status_line: self.status_line.expect("Setting a status line is required."),
            headers: self.headers.expect("Setting at least one header is required."),
            body: self.body,
        }
    }

    pub fn header(mut self: Self, name: String, value: String) -> Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }

        self.headers.as_mut().unwrap().insert(name, value);
        dbg!(&self.headers);

        self
    }

    pub fn status_line(mut self: Self, status_line: StatusLine) -> Self {
        self.status_line = Some(status_line);
        self
    }
}
