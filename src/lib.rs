mod response;
mod request;

use std::collections::HashMap;
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
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Length"), body.len().to_string());
    headers.insert(String::from("Content-Type"), String::from("text/html"));

    let response = Response {
        status_line: StatusLine::from_string("HTTP/1.1 200 OK".to_string()),
        headers: Some(headers),
        body: Some(body),
    };

    stream.write_all(response.as_bytes().as_slice()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
