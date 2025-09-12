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

    pub fn get_path(self: Self) -> String {
        self.path
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_delete_method() {
        let row = "DELETE / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Delete, result);
    }

    #[test]
    fn should_parse_get_method() {
        let row = "GET / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Get, result);
    }
    
    #[test]
    fn should_parse_head_method() {
        let row = "HEAD / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Head, result);
    }
    
    #[test]
    fn should_parse_options_method() {
        let row = "OPTIONS / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Options, result);
    }
    
    #[test]
    fn should_parse_patch_method() {
        let row = "PATCH / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Patch, result);
    }
    
    #[test]
    fn should_parse_post_method() {
        let row = "POST / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Post, result);
    }
    
    #[test]
    fn should_parse_put_method() {
        let row = "PUT / HTTP/1.1";
        let result = RequestLine::parse_method(row);

        assert_eq!(RequestMethod::Put, result);
    }

    #[test]
    #[should_panic]
    fn should_not_parse_method_of_unknown_type() {
        let row = "ASDF / HTTP/1.1";
        let result = RequestLine::parse_method(row);
    }

    #[test]
    fn should_parse_path() {
        let s = "/";
        let result = RequestLine::parse_path(s);

        assert_eq!(result, s);
    }

    #[test]
    fn should_parse_longer_path() {
        let s = "/some/longer/path";
        let result = RequestLine::parse_path(s);

        assert_eq!(result, s);
    }

    #[test]
    fn should_parse_absolute_uri_path() {
        let s = "http://www.example.com/index/asdf";
        let result = RequestLine::parse_path(s);

        assert_eq!(result, s);
    }

    #[test]
    fn should_parse_path_with_query() {
        let s = "/submit-form?user=alice";
        let expected = "/submit-form";
        let result = RequestLine::parse_path(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_query_if_present() {
        let s = "/submit-form?user=alice";
        let expected = "user=alice";
        let result = RequestLine::parse_query(s);

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn should_not_parse_query_if_not_present() {
        let s = "/submit-form";
        let result = RequestLine::parse_query(s);
    }
}
