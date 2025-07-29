pub mod request {
    use std::collections::HashMap;
    use std::io::{BufReader, prelude::*};
    use std::net::TcpStream;

    #[derive(Debug)]
    pub struct Request {
        method: Option<RequestMethod>,
        endpoint: Option<String>,
    }

    impl Request {
        pub fn build(self: Self) -> Self {
            Request {
                method: self.method,
                endpoint: self.endpoint,
            }
        }

        pub fn builder() -> Self {
            Request {
                method: None,
                endpoint: None,
            }
        }

        pub fn endpoint(mut self: Self, endpoint: String) -> Self {
            self.endpoint = Some(endpoint);
            self
        }

        pub fn method(mut self: Self, method: RequestMethod) -> Self {
            self.method = Some(method);
            self
        }

        pub fn parse_from_tcp_stream(stream: &TcpStream) -> Self {
            let mut buf_reader = BufReader::new(stream);
            let request_vec: Vec<String> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            dbg!(&request_vec);

            let request_line = &request_vec[0];

            let mut headers: HashMap<String, String> = HashMap::new();
            for line in &request_vec[1..] {
                if let Some((key, value)) = line.split_once(": ") {
                    headers.insert(key.into(), value.into());
                }
            }

            dbg!(&headers);

            let content_length = headers
                .get("Content-Length")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0);

            let mut body = vec![0; content_length];
            buf_reader.read_exact(&mut body).unwrap();

            dbg!(body);

            Request {
                method: Self::parse_method(&request_line),
                endpoint: Self::parse_endpoint(&request_line),
            }
        }

        fn parse_endpoint(request_line: &String) -> Option<String> {
            let endpoint: String = request_line.split(" ").collect::<Vec<_>>()[1].into();

            Some(endpoint)
        }

        fn parse_method(request_line: &String) -> Option<RequestMethod> {
            let method = match request_line {
                x if x.to_lowercase().contains("delete") => RequestMethod::Delete,
                x if x.to_lowercase().contains("get") => RequestMethod::Get,
                x if x.to_lowercase().contains("head") => RequestMethod::Head,
                x if x.to_lowercase().contains("options") => RequestMethod::Options,
                x if x.to_lowercase().contains("patch") => RequestMethod::Patch,
                x if x.to_lowercase().contains("post") => RequestMethod::Post,
                x if x.to_lowercase().contains("put") => RequestMethod::Put,
                _ => panic!("Should have matched request method")
            };

            Some(method)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum RequestMethod {
        Delete,
        Get,
        Head,
        Options,
        Patch,
        Post,
        Put,
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
}