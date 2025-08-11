mod request_line;
mod request_method;

use std::collections::HashMap;
use std::io::{BufReader, prelude::*};
use std::net::TcpStream;

use crate::request::request_line::RequestLine;

#[derive(Debug)]
pub struct Request {
    request_line: RequestLine,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl Request {
    pub fn parse_from_tcp_stream(stream: &TcpStream) -> Self {
        let mut buf_reader = BufReader::new(stream);
        let request_vec: Vec<String> = buf_reader
            .by_ref()
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        
        let request_line = RequestLine::from_string(&request_vec[0]);

        let headers = Self::parse_headers(&request_vec[1..]);

        let content_length: usize = match headers.is_some() {
            true => {
                headers
                .as_ref()
                .unwrap()
                .get("Content-Length")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0)
            },
            false => 0
        };
        
        Request {
            request_line: request_line,
            headers: headers,
            body: Self::read_body(content_length, &mut buf_reader),
        }
    }

    fn read_body(content_length: usize, reader: &mut BufReader<&TcpStream>) -> Option<String> {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        let body: String = String::from_utf8_lossy(&body).into();

        let body = if body.is_empty() {None} else {Some(body)};
        body
    }

    fn parse_endpoint(request_line: &String) -> String {
        let endpoint: String = request_line.split(" ").collect::<Vec<_>>()[1].into();
        endpoint
    }

    fn parse_headers(request_vec: &[String]) -> Option<HashMap<String, String>> {
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in request_vec {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.into(), value.into());
            }
        }

        if headers.is_empty() {None} else {Some(headers)}
    }
}
