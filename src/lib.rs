pub mod controllers;
pub mod response;
mod request;

use httpdate::fmt_http_date;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use std::net::{TcpStream};
use std::time::SystemTime;

use crate::request::*;
use crate::response::*;

pub fn handle_client(mut stream: TcpStream, registry: &HashMap<&'static str, fn() -> Response>) {
    let request = Request::parse_from_tcp_stream(&stream);
    dbg!(&request);

    let path = request.get_path();

    let response = match registry.get(path.as_str()) {
        Some(handler) => handler(),
        None => Response::builder()._404().build()
    };

    dbg!(&response);

    stream.write_all(response.as_bytes().as_slice()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
