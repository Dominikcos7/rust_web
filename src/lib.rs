mod response;
mod request;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpStream};

use crate::request::*;
use crate::response::*;

pub fn handle_client(mut stream: TcpStream) {
    let request = Request::parse_from_tcp_stream(&stream);
    dbg!(request);

    let response = Response {
        status_line: StatusLine::from_string("HTTP/1.1 200 OK".to_string()),
        headers: None,
        body: None,
    };

    stream.write_all(response.as_bytes().as_slice()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
