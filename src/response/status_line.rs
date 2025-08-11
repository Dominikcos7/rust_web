#[derive(Debug)]
pub struct StatusLine {
    http_version: String,
    status_code: i32,
    reason_phrase: String,
}