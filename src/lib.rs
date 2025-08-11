mod response;
mod request;

use std::io::prelude::*;
use std::net::{TcpStream};

use crate::request::*;

pub fn handle_client(stream: TcpStream) {
    let request = Request::parse_from_tcp_stream(&stream);
    dbg!(request);

    resp_200(stream)
}

fn resp_200(mut stream: TcpStream) {
    let response = format!("HTTP/1.1 200 OK\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
