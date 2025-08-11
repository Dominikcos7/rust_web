use std::collections::HashMap;

mod status_line;

#[derive(Debug)]
pub struct Response {
    status_line: StatusLine,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}
