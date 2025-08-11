use super::request_method::RequestMethod;

#[derive(Debug)]
pub struct RequestLine {
    method: RequestMethod,
    path: String,
    query: String,
    http_version: String,
}

impl RequestLine {
    pub fn from_string(s: &String) -> Self {
        RequestLine {
            method: Self::parse_method(s),
            path: String::from(""),
            query: String::from(""),
            http_version: String::from(""),
        }
    }

    fn parse_method(request_line: &String) -> RequestMethod {
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

        method
    }
}
