pub mod controllers;
pub mod registry;
pub mod response;
mod request;

use std::io::prelude::*;
use std::net::{TcpStream};

use crate::registry::Registry;
use crate::request::*;
use crate::response::*;

pub fn handle_client(mut stream: TcpStream, registry: &Registry) {
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
