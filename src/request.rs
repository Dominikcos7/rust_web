mod request_line;
mod request_method;

use std::collections::HashMap;
use std::io::{BufReader, prelude::*};
use std::net::TcpStream;

use crate::request::request_line::RequestLine;

#[derive(Debug)]
pub struct Request {
    request_line: Option<RequestLine>,
    body: Option<String>,
}

impl Request {
    pub fn build(self: Self) -> Self {
        Request {
            request_line: self.request_line,
            body: self.body,
        }
    }

    pub fn builder() -> Self {
        Request {
            request_line: None,
            body: None,
        }
    }

    pub fn body(mut self: Self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn request_line(mut self: Self, request_line: RequestLine) -> Self {
        self.request_line = Some(request_line);
        self
    }

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

        let content_type = headers
            .get("Content-Type")
            .and_then(|v| v.parse::<String>().ok())
            .unwrap_or(String::from(""));
        
        let content_length = headers
            .get("Content-Length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        
        Request {
            request_line: Some(request_line),
            body: Self::parse_body(content_type, content_length, &mut buf_reader),
        }
    }

    fn parse_body(content_type: String, content_length: usize, reader: &mut BufReader<&TcpStream>) -> Option<String> {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        let body: String = String::from_utf8_lossy(&body).into();

        let body = if body.is_empty() {None} else {Some(body)};
        body
    }

    fn parse_endpoint(request_line: &String) -> Option<String> {
        let endpoint: String = request_line.split(" ").collect::<Vec<_>>()[1].into();

        Some(endpoint)
    }

    fn parse_headers(request_vec: &[String]) -> HashMap<String, String> {
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in request_vec {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.into(), value.into());
            }
        }

        headers
    }
}

#[cfg(test)]
mod tests {
    use crate::request::request::RequestMethod;

    use super::*;

    #[test]
    fn should_parse_empty_endpoint() {
        let row = String::from("DELETE / HTTP/1.1");
        let endpoint = Request::parse_endpoint(&row).unwrap();

        assert_eq!(endpoint, "/".to_string())
    }

    fn should_parse_longer_endpoint() {
        let row = String::from("DELETE /some/longer/endpoint HTTP/1.1");
        let endpoint = Request::parse_endpoint(&row).unwrap();

        assert_eq!(endpoint, "/some/longer/endpoint".to_string())
    }

    #[test]
    fn should_parse_method_of_delete_request() {
        let row = String::from("DELETE / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Delete)
    }

    #[test]
    fn should_parse_method_of_get_request() {
        let row = String::from("GET / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Get)
    }
    
    #[test]
    fn should_parse_method_of_head_request() {
        let row = String::from("HEAD / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Head)
    }
    
    #[test]
    fn should_parse_method_of_options_request() {
        let row = String::from("OPTIONS / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Options)
    }
    
    #[test]
    fn should_parse_method_of_patch_request() {
        let row = String::from("PATCH / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Patch)
    }
    
    #[test]
    fn should_parse_method_of_post_request() {
        let row = String::from("POST / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Post)
    }
    
    #[test]
    fn should_parse_method_of_put_request() {
        let row = String::from("PUT / HTTP/1.1");
        let method = Request::parse_method(&row).unwrap();

        assert_eq!(method, RequestMethod::Put)
    }

    #[test]
    #[should_panic]
    fn should_not_parse_method_of_unknown_request() {
        let row = String::from("ASDF / HTTP/1.1");
        let method = Request::parse_method(&row);
    }
}
