mod response;
mod request;

use std::io::prelude::*;
use std::net::{TcpStream};

use crate::request::*;
use crate::response::*;

//todo: insert an url mapper into this function
//      the mapper will find the controller which will send the response
pub fn handle_client(mut stream: TcpStream) {
    let request = Request::parse_from_tcp_stream(&stream);
    dbg!(request);

    let body = String::from("some body");
    let response = Response::builder()
        .status_line(StatusLine::from_string("HTTP/1.1 200 OK".to_string()))
        .header(String::from("Content-Length"), body.len().to_string())
        .header(String::from("Content-Type"), String::from("text/html"))
        .body(body)
        .build();

    stream.write_all(response.as_bytes().as_slice()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
