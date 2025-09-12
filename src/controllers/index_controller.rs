use crate::Response;
use crate::StatusLine;

use httpdate::fmt_http_date;
use std::fs;
use std::time::SystemTime;

pub struct IndexController {}

impl IndexController {
    pub fn action_index() -> Response {
        println!("Index controller index action called");
    
        let body = fs::read_to_string("./src/views/index/index.html").expect("Should have found file.");
        let response = Response::builder()
            .status_line(StatusLine::from_str("HTTP/1.1 200 OK"))
            .header("Date", &fmt_http_date(SystemTime::now()))
            .header("Content-Length", &body.len().to_string())
            .header("Content-Type", "text/html")
            .body(body)
            .build();

        response
    }
}