use super::request_method::RequestMethod;

#[derive(Debug)]
pub struct RequestLine {
    method: RequestMethod,
    path: String,
    query: Option<String>,
    http_version: String,
}

impl RequestLine {
    pub fn from_string(request_line: &String) -> Self {
        let parts = request_line.split(" ").collect::<Vec<_>>();

        RequestLine {
            method: Self::parse_method(parts[0]),
            path: Self::parse_path(parts[1]),
            query: if parts[1].contains("?") {Some(Self::parse_query(parts[1]))} else {None},
            http_version: parts[2].into(),
        }
    }

    fn parse_method(request_line: &str) -> RequestMethod {
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

    fn parse_path(s: &str) -> String {
        let path: String = s.split("?").collect::<Vec<_>>()[0].into();
        path
    }

    fn parse_query(s: &str) -> String {
        let query: String = s.split("?").collect::<Vec<_>>()[1].into();
        query
    }
}
