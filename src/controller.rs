pub mod dummy_controller;

use httpdate::fmt_http_date;
use std::time::SystemTime;

use crate::response::{Response, StatusLine};

pub trait Controller {
    fn respond_404_not_found() -> Response where Self: Sized {
        let response = Response::builder()
        .status_line(StatusLine::from_str("HTTP/1.1 404 Not Found"))
        .header("Date", &fmt_http_date(SystemTime::now()))
        .header("Content-Length", "0")
        .header("Content-Type", "text/html")
        .build();

        response
    }
}