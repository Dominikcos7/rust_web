pub use crate::response::status_line::StatusLine;

use std::collections::HashMap;

mod status_line;

#[derive(Debug)]
pub struct Response {
    pub status_line: StatusLine,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl Response {
    pub fn as_bytes(self: &Self) -> Vec<u8> {
        let mut s: String = self.status_line.as_string();

        if self.headers.is_some() {
            for (header, value) in self.headers.as_ref().unwrap() {
                s.push_str(format!("{header}: {value}").as_str());
            }
        }

        s.push_str("\r\n\r\n");
        
        if self.body.is_some() {
            s.push_str(self.body.as_ref().unwrap().as_str());
        }

        dbg!(&s);
        s.into_bytes()
    }
}
